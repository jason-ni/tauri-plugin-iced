// Utility functions module
// This will contain helper functions

use crate::event_conversion::{convert_mouse_button, convert_mouse_position, create_viewport};
use crate::renderer::Renderer;
use crate::IcedControls;
use iced_core::mouse;
use iced_wgpu::graphics::Viewport;
use iced_winit::core::Event;
use iced_winit::runtime::user_interface::{Cache, UserInterface};
use iced_winit::Clipboard;
use tauri_runtime::dpi::PhysicalSize;
use tauri_runtime_wry::tao::event::WindowEvent;

/// A single Iced window with its associated state.
///
/// Each window has independent state and rendering resources.
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
}

unsafe impl<M, C: IcedControls<Message = M>> Send for IcedWindow<M, C> {}
unsafe impl<M, C: IcedControls<Message = M>> Sync for IcedWindow<M, C> {}

/// Check if a Tauri window event is relevant for Iced.
///
/// Only forward these events to Iced:
/// - CursorMoved: Mouse position changes
/// - MouseInput: Button clicks/releases
/// - ModifiersChanged: Modifier key state
/// - Resized: Window dimension changes
/// - ScaleFactorChanged: DPI scale changes
///
/// Ignore these events:
/// - Moved: Window position changes
/// - Focused: Focus state
/// - DroppedFile: File drops
/// - HoveredFile: File hover
/// - Touch events: Not implemented for desktop
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
    /// Process a Tauri window event and convert it to an Iced event if relevant.
    ///
    /// Returns true if event was consumed (should not be forwarded).
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
                self.viewport = create_viewport(new_size.width, new_size.height, self.scale_factor);
                true
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor = *scale_factor as f32;
                self.viewport =
                    create_viewport(self.size.width, self.size.height, self.scale_factor);
                true
            }
            _ => false,
        }
    }

    /// Process accumulated Iced events and handle user interactions.
    pub fn process_events(&mut self) {
        if self.events.is_empty() {
            return;
        }

        let messages = std::mem::take(&mut self.events);

        // Forward events to controls
        for event in &messages {
            self.controls.handle_event(event);
        }

        let mut interface = UserInterface::build(
            self.controls.view(),
            self.viewport.logical_size(),
            std::mem::take(&mut self.cache),
            &mut self.renderer.renderer,
        );

        let (_, _control_messages) = interface.update(
            &messages,
            self.cursor,
            &mut self.renderer.renderer,
            &mut self.clipboard,
            &mut std::vec::Vec::new(),
        );

        self.cache = interface.into_cache();

        // Note: Message handling would go here if we weren't using Message = ()
    }

    /// Render the UI to the WGPU surface.
    pub fn render(&mut self) {
        let mut interface = UserInterface::build(
            self.controls.view(),
            self.viewport.logical_size(),
            std::mem::take(&mut self.cache),
            &mut self.renderer.renderer,
        );

        let (_, _) = interface.update(
            &[Event::Window(iced_core::window::Event::RedrawRequested(
                iced_core::time::Instant::now(),
            ))],
            self.cursor,
            &mut self.renderer.renderer,
            &mut self.clipboard,
            &mut std::vec::Vec::new(),
        );

        interface.draw(
            &mut self.renderer.renderer,
            &iced_winit::core::Theme::Dark,
            &iced_core::renderer::Style::default(),
            self.cursor,
        );

        self.cache = interface.into_cache();

        // TODO: Call renderer.present() to render to surface
        // This requires getting the current texture from the surface
    }
}
