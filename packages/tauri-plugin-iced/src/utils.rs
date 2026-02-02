use crate::event_conversion::{convert_modifiers, convert_window_event, create_viewport};
use crate::renderer::Renderer;
use crate::scene::{clear, Scene};
use crate::{IcedControls, convert_mouse_position};
use anyhow::Error;
use iced_core::keyboard;
use iced_core::mouse;
use iced_wgpu::graphics::Viewport;
use iced_winit::core::Event;
use iced_winit::runtime::user_interface::{Cache, State, UserInterface};
use iced_winit::Clipboard;
use tauri::AppHandle;
use tauri_runtime::dpi::PhysicalSize;
use tauri_runtime_wry::tao::event::WindowEvent;

// Type alias for mouse interaction (cursor icon)
pub type MouseInteraction = mouse::Interaction;

pub struct IcedWindow<M> {
    pub label: String,
    pub controls: Box<dyn IcedControls<Message = M> + Send + Sync>,
    pub renderer: Renderer,
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
                self.cursor = mouse::Cursor::Available(convert_mouse_position(position.x, position.y, self.scale_factor))
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

        let mut interface = UserInterface::build(
            self.controls.view(),
            self.viewport.logical_size(),
            std::mem::take(&mut self.cache),
            self.renderer.iced_renderer(),
        );

        let (state, _) = interface.update(
            &messages,
            self.cursor,
            self.renderer.iced_renderer(),
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
        if self.resized {
            self.renderer.gpu.resize(self.size.width, self.size.height);
            self.viewport = create_viewport(self.size.width, self.size.height, self.scale_factor);
            self.resized = false;
        }

        let frame_and_view = self.renderer.gpu.get_frame()?;

        let mut encoder = self
            .renderer
            .gpu
            .device()
            .create_command_encoder(&iced_wgpu::wgpu::CommandEncoderDescriptor { label: None });

        if let Some(scene) = &self.scene {
            let mut render_pass = clear(
                &frame_and_view.view,
                &mut encoder,
                self.controls.background_color(),
            );
            scene.draw(&mut render_pass);
        }

        self.renderer.gpu.queue().submit([encoder.finish()]);

        let mut interface = UserInterface::build(
            self.controls.view(),
            self.viewport.logical_size(),
            std::mem::take(&mut self.cache),
            self.renderer.iced_renderer(),
        );

        let (state, _) = interface.update(
            &[Event::Window(iced_core::window::Event::RedrawRequested(
                iced_core::time::Instant::now(),
            ))],
            self.cursor,
            self.renderer.iced_renderer(),
            &mut self.clipboard,
            &mut std::vec::Vec::new(),
        );

        interface.draw(
            self.renderer.iced_renderer(),
            &iced_winit::core::Theme::Dark,
            &iced_core::renderer::Style::default(),
            self.cursor,
        );

        self.cache = interface.into_cache();

        if let iced::Renderer::Primary(wgpu_renderer) = self.renderer.iced_renderer() {
            wgpu_renderer.present(
            None,
            frame_and_view.surface_texture.texture.format(),
            &frame_and_view.view,
            &self.viewport,
            );
        }

        frame_and_view.surface_texture.present();

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
                if let Some(surface_error) = e.downcast_ref::<iced_wgpu::wgpu::SurfaceError>() {
                    if surface_error == &iced_wgpu::wgpu::SurfaceError::OutOfMemory {
                        panic!("Swapchain error: {surface_error}. Rendering cannot continue.")
                    }
                }
                None
            }
        }
    }
}
