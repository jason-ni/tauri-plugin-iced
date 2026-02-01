use crate::event_conversion::{convert_mouse_button, convert_mouse_position, create_viewport};
use crate::renderer::Renderer;
use crate::scene::{clear, Scene};
use crate::IcedControls;
use anyhow::Error;
use iced_core::mouse;
use iced_wgpu::graphics::Viewport;
use iced_winit::core::Event;
use iced_winit::runtime::user_interface::{Cache, UserInterface};
use iced_winit::Clipboard;
use tauri::AppHandle;
use tauri_runtime::dpi::PhysicalSize;
use tauri_runtime_wry::tao::event::WindowEvent;

pub struct IcedWindow<M, C: IcedControls<Message = M>> {
    pub label: String,
    pub controls: C,
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
}

unsafe impl<M, C: IcedControls<Message = M>> Send for IcedWindow<M, C> {}
unsafe impl<M, C: IcedControls<Message = M>> Sync for IcedWindow<M, C> {}

fn is_relevant_event(event: &WindowEvent) -> bool {
    matches!(
        event,
        WindowEvent::CursorMoved { .. }
            | WindowEvent::MouseInput { .. }
            | WindowEvent::ModifiersChanged(_)
            | WindowEvent::Resized(_)
            | WindowEvent::ScaleFactorChanged { .. }
    )
}

impl<M, C: IcedControls<Message = M>> IcedWindow<M, C> {
    pub fn handle_event(&mut self, event: &WindowEvent) -> bool {
        if !is_relevant_event(event) {
            return false;
        }

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(cursor_pos) =
                    convert_mouse_position(position.x, position.y, self.scale_factor)
                {
                    self.cursor = mouse::Cursor::Available(cursor_pos);
                    self.events.push(Event::Mouse(mouse::Event::CursorMoved {
                        position: cursor_pos,
                    }));
                    true
                } else {
                    false
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if let Some(mouse_button) = convert_mouse_button(button) {
                    let mouse_event =
                        if *state == tauri_runtime_wry::tao::event::ElementState::Pressed {
                            mouse::Event::ButtonPressed(mouse_button)
                        } else {
                            mouse::Event::ButtonReleased(mouse_button)
                        };
                    self.events.push(Event::Mouse(mouse_event));
                    true
                } else {
                    false
                }
            }
            WindowEvent::ModifiersChanged(new_modifiers) => {
                let state = new_modifiers.bits();
                let modifiers = iced_core::keyboard::Modifiers::from_bits_retain(state);
                self.events.push(Event::Keyboard(
                    iced_core::keyboard::Event::ModifiersChanged(modifiers),
                ));
                true
            }
            WindowEvent::Resized(new_size) => {
                self.size = PhysicalSize::new(new_size.width, new_size.height);
                self.resized = true;
                true
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor = *scale_factor as f32;
                self.resized = true;
                true
            }
            _ => false,
        }
    }

    pub fn process_events(&mut self) {
        if self.events.is_empty() {
            return;
        }

        let messages = std::mem::take(&mut self.events);

        for event in &messages {
            self.controls.handle_event(event);
        }

        let mut interface = UserInterface::build(
            self.controls.view(),
            self.viewport.logical_size(),
            std::mem::take(&mut self.cache),
            self.renderer.iced_renderer(),
        );

        let mut control_messages = std::vec::Vec::new();

        let _ = interface.update(
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
    }

    pub fn render(&mut self, _app_handle: &AppHandle) -> Result<(), Error> {
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

        let (_, _) = interface.update(
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

        self.renderer.iced_renderer().present(
            None,
            frame_and_view.surface_texture.texture.format(),
            &frame_and_view.view,
            &self.viewport,
        );

        frame_and_view.surface_texture.present();

        Ok(())
    }

    pub fn render_with_retry(&mut self, app_handle: &AppHandle) {
        match self.render(app_handle) {
            Ok(_) => {}
            Err(e) => {
                if let Some(surface_error) = e.downcast_ref::<iced_wgpu::wgpu::SurfaceError>() {
                    match surface_error {
                        iced_wgpu::wgpu::SurfaceError::OutOfMemory => {
                            panic!("Swapchain error: {surface_error}. Rendering cannot continue.")
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
