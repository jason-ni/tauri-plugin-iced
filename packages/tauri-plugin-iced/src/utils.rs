use crate::event_conversion::{convert_modifiers, convert_window_event, create_viewport};
use crate::renderer::IcedRenderer;
use crate::scene::Scene;
use crate::{convert_mouse_position, IcedControls};
use anyhow::Error;
use iced_core::keyboard;
use iced_core::mouse;
use iced_tiny_skia::graphics::Viewport;
use iced_winit::core::{Event, Rectangle};
use iced_winit::runtime::user_interface::{Cache, State, UserInterface};
use iced_winit::Clipboard;
use tauri::AppHandle;
use tauri_runtime::dpi::PhysicalSize;
use tauri_runtime_wry::tao::event::WindowEvent;

// Type alias for mouse interaction (cursor icon)
pub type MouseInteraction = mouse::Interaction;

pub struct IcedWindow<M> {
    pub label: String,
    pub window: tauri::Window,
    pub controls: Box<dyn IcedControls<Message = M> + Send + Sync>,
    pub renderer: Option<IcedRenderer>,
    pub viewport: Viewport,
    pub events: Vec<Event>,
    pub cache: Cache,
    pub clipboard: Clipboard,
    pub cursor: mouse::Cursor,
    pub scale_factor: f32,
    pub size: PhysicalSize<u32>,
    pub scene: Option<Box<dyn Scene>>,
    pub resized: bool,
    pub modifiers: keyboard::Modifiers,
}

unsafe impl<M> Send for IcedWindow<M> {}
unsafe impl<M> Sync for IcedWindow<M> {}

fn is_relevant_event(event: &WindowEvent) -> bool {
    match event {
        WindowEvent::CursorMoved { .. }
        | WindowEvent::MouseInput { .. }
        | WindowEvent::MouseWheel { .. }
        | WindowEvent::ModifiersChanged(_)
        | WindowEvent::Resized(_)
        | WindowEvent::ScaleFactorChanged { .. }
        | WindowEvent::KeyboardInput { .. }
        | WindowEvent::Focused(_) => true,
        _ => false,
    }
}

impl<M> IcedWindow<M> {
    pub fn handle_event(&mut self, event: &WindowEvent) -> bool {
        if !is_relevant_event(event) {
            return false;
        }

        match event {
            WindowEvent::ModifiersChanged(new_modifiers) => {
                self.modifiers = convert_modifiers(&new_modifiers);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor = mouse::Cursor::Available(convert_mouse_position(
                    position.x,
                    position.y,
                    self.scale_factor,
                ))
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor = *scale_factor as f32;
                self.resized = true;
            }
            WindowEvent::Resized(new_size) => {
                self.size = PhysicalSize::new(new_size.width, new_size.height);
                self.resized = true;
            }
            _ => {}
        }

        if let Some(iced_event) = convert_window_event(event, self.scale_factor, self.modifiers) {
            self.events.push(iced_event);
            true
        } else {
            false
        }
    }

    pub fn process_events(&mut self) -> Option<MouseInteraction> {
        if self.events.is_empty() {
            return None;
        }

        let messages = std::mem::take(&mut self.events);

        let mut control_messages = std::vec::Vec::new();

        let renderer = self.renderer.as_mut().expect("Renderer not initialized");

        let mut interface = UserInterface::build(
            self.controls.view(),
            self.viewport.logical_size(),
            std::mem::take(&mut self.cache),
            renderer.tiny_skia_renderer(),
        );

        let (state, _) = interface.update(
            &messages,
            self.cursor,
            renderer.tiny_skia_renderer(),
            &mut self.clipboard,
            &mut control_messages,
        );

        self.cache = interface.into_cache();
        for message in control_messages {
            self.controls.update(message);
        }

        // Return mouse interaction for cursor updates
        if let State::Updated {
            mouse_interaction, ..
        } = state
        {
            Some(mouse_interaction)
        } else {
            None
        }
    }

    pub fn render(&mut self, _app_handle: &AppHandle) -> Result<Option<MouseInteraction>, Error> {
        let renderer = self.renderer.as_mut().expect("Renderer not initialized");

        // Handle resize by updating surface and viewport
        if self.resized {
            renderer
                .surface_resource
                .resize(self.size.width, self.size.height);
            self.viewport = create_viewport(self.size.width, self.size.height, self.scale_factor);
            self.resized = false;
        }

        // CPU rendering pipeline:
        // 1. Get mutable pixel buffer from softbuffer surface
        // 2. Create tiny_skia drawing target from buffer bytes
        // 3. Build and update Iced UI
        // 4. Composit Iced UI layers to pixel buffer
        // 5. Draw custom scene on top (if exists)
        // 6. Present buffer to window

        let mut buffer = renderer
            .surface_resource
            .get_buffer_mut()
            .map_err(|e| anyhow::anyhow!("Failed to get buffer: {}", e))?;
        let width = buffer.width().get();
        let height = buffer.height().get();

        let mut pixels =
            tiny_skia::PixmapMut::from_bytes(bytemuck::cast_slice_mut(&mut buffer), width, height)
                .expect("Create pixel map");

        let tiny_skia_renderer = &mut renderer.renderer;

        let mut interface = UserInterface::build(
            self.controls.view(),
            self.viewport.logical_size(),
            std::mem::take(&mut self.cache),
            tiny_skia_renderer,
        );

        let (state, _) = interface.update(
            &[Event::Window(iced_core::window::Event::RedrawRequested(
                iced_core::time::Instant::now(),
            ))],
            self.cursor,
            tiny_skia_renderer,
            &mut self.clipboard,
            &mut std::vec::Vec::new(),
        );

        // Draw Iced UI to populate renderer layers (no GPU operations yet)
        interface.draw(
            tiny_skia_renderer,
            &iced_winit::core::Theme::Dark,
            &iced_core::renderer::Style::default(),
            self.cursor,
        );

        self.cache = interface.into_cache();

        // Composit Iced UI layers to CPU pixel buffer using tiny_skia
        // This performs CPU rasterization of all UI elements
        tiny_skia_renderer.draw(
            &mut pixels,
            &mut tiny_skia::Mask::new(width, height).expect("Create mask"),
            &self.viewport,
            &[Rectangle::with_size(self.viewport.logical_size())],
            self.controls.background_color(),
        );

        // Draw custom scene on top of Iced UI (CPU rendering)
        if let Some(scene) = &self.scene {
            scene.draw(&mut pixels, self.controls.background_color());
        }

        // Present pixel buffer to window (displays on screen)
        buffer
            .present()
            .map_err(|e| anyhow::anyhow!("Failed to present buffer: {}", e))?;

        if let State::Updated {
            mouse_interaction, ..
        } = state
        {
            Ok(Some(mouse_interaction))
        } else {
            Ok(None)
        }
    }
    pub fn render_with_retry(&mut self, app_handle: &AppHandle) -> Option<MouseInteraction> {
        match self.render(app_handle) {
            Ok(mouse_interaction) => mouse_interaction,
            Err(e) => {
                log::warn!("Render error: {}", e);
                None
            }
        }
    }
}

pub fn set_window_transparent(window: &tauri::Window) {
    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::{NSColor, NSWindow};
        use cocoa::base::{id, nil};

        let ns_window = window.ns_window().unwrap() as id;
        unsafe {
            let bg_color = NSColor::colorWithRed_green_blue_alpha_(
                nil,
                0.0 / 255.0,
                0.0 / 255.0,
                0.0 / 255.0,
                0.0,
            );
            ns_window.setBackgroundColor_(bg_color);
        }
    }
}
