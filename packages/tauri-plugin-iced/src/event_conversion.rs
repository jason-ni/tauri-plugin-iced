// Event conversion module
// This will contain Tauri → Iced event mapping

use iced_core::keyboard;
use iced_core::mouse;
use iced_core::Size;
use iced_core::SmolStr;
use iced_wgpu::graphics::Viewport;
use iced_winit::core::{Event, Point};
use tauri_runtime_wry::tao::event::{ElementState, MouseButton, WindowEvent as TaoWindowEvent};
use tauri_runtime_wry::tao::keyboard::{Key, KeyCode, KeyLocation, ModifiersState};

/// Convert physical mouse position to logical position with scale factor.
pub fn convert_mouse_position(
    physical_x: f64,
    physical_y: f64,
    scale_factor: f32,
) -> Point {
    Point::new(
        physical_x as f32 / scale_factor,
        physical_y as f32 / scale_factor,
    )
}

/// Convert Tauri mouse button to Iced pointer button.
pub fn convert_mouse_button(button: &MouseButton) -> Option<mouse::Button> {
    match button {
        MouseButton::Left => Some(mouse::Button::Left),
        MouseButton::Right => Some(mouse::Button::Right),
        MouseButton::Middle => Some(mouse::Button::Middle),
        MouseButton::Other(v) => Some(mouse::Button::Other(*v)),
        _ => None,
    }
}

/// Create a viewport from physical dimensions and scale factor.
pub fn create_viewport(physical_width: u32, physical_height: u32, scale_factor: f32) -> Viewport {
    Viewport::with_physical_size(Size::new(physical_width, physical_height), scale_factor)
}

/// Convert Tao logical key to Iced key.
pub fn convert_key(key: &Key) -> keyboard::Key {
    use keyboard::key::Named;

    match key {
        Key::Character(c) => keyboard::Key::Character(SmolStr::new(c)),
        Key::Alt => keyboard::Key::Named(Named::Alt),
        Key::AltGraph => keyboard::Key::Named(Named::AltGraph),
        Key::CapsLock => keyboard::Key::Named(Named::CapsLock),
        Key::Control => keyboard::Key::Named(Named::Control),
        Key::Fn => keyboard::Key::Named(Named::Fn),
        Key::FnLock => keyboard::Key::Named(Named::FnLock),
        Key::NumLock => keyboard::Key::Named(Named::NumLock),
        Key::ScrollLock => keyboard::Key::Named(Named::ScrollLock),
        Key::Shift => keyboard::Key::Named(Named::Shift),
        Key::Symbol => keyboard::Key::Named(Named::Symbol),
        Key::SymbolLock => keyboard::Key::Named(Named::SymbolLock),
        Key::Hyper => keyboard::Key::Named(Named::Hyper),
        Key::Super => keyboard::Key::Named(Named::Super),
        Key::Enter => keyboard::Key::Named(Named::Enter),
        Key::Tab => keyboard::Key::Named(Named::Tab),
        Key::Space => keyboard::Key::Named(Named::Space),
        Key::ArrowDown => keyboard::Key::Named(Named::ArrowDown),
        Key::ArrowLeft => keyboard::Key::Named(Named::ArrowLeft),
        Key::ArrowRight => keyboard::Key::Named(Named::ArrowRight),
        Key::ArrowUp => keyboard::Key::Named(Named::ArrowUp),
        Key::End => keyboard::Key::Named(Named::End),
        Key::Home => keyboard::Key::Named(Named::Home),
        Key::PageDown => keyboard::Key::Named(Named::PageDown),
        Key::PageUp => keyboard::Key::Named(Named::PageUp),
        Key::Backspace => keyboard::Key::Named(Named::Backspace),
        Key::Delete => keyboard::Key::Named(Named::Delete),
        Key::Insert => keyboard::Key::Named(Named::Insert),
        Key::Escape => keyboard::Key::Named(Named::Escape),
        _ => keyboard::Key::Unidentified,
    }
}

/// Convert Tao KeyCode to Iced physical key.
pub fn convert_physical_key(code: &KeyCode) -> keyboard::key::Physical {
    use keyboard::key::Code;

    match code {
        KeyCode::Backspace => keyboard::key::Physical::Code(Code::Backspace),
        KeyCode::Enter => keyboard::key::Physical::Code(Code::Enter),
        KeyCode::Space => keyboard::key::Physical::Code(Code::Space),
        KeyCode::Tab => keyboard::key::Physical::Code(Code::Tab),
        KeyCode::ArrowUp => keyboard::key::Physical::Code(Code::ArrowUp),
        KeyCode::ArrowDown => keyboard::key::Physical::Code(Code::ArrowDown),
        KeyCode::ArrowLeft => keyboard::key::Physical::Code(Code::ArrowLeft),
        KeyCode::ArrowRight => keyboard::key::Physical::Code(Code::ArrowRight),
        _ => keyboard::key::Physical::Unidentified(keyboard::key::NativeCode::Unidentified),
    }
}

/// Convert Tao Location to Iced Location.
pub fn convert_location(location: &KeyLocation) -> keyboard::Location {
    match location {
        KeyLocation::Standard => keyboard::Location::Standard,
        KeyLocation::Left => keyboard::Location::Left,
        KeyLocation::Right => keyboard::Location::Right,
        KeyLocation::Numpad => keyboard::Location::Numpad,
        _ => keyboard::Location::Standard,
    }
}

/// Convert Tao modifiers to Iced modifiers.
pub fn convert_modifiers(modifiers: &ModifiersState) -> keyboard::Modifiers {
    let mut result = keyboard::Modifiers::empty();
    result.set(keyboard::Modifiers::SHIFT, modifiers.shift_key());
    result.set(keyboard::Modifiers::CTRL, modifiers.control_key());
    result.set(keyboard::Modifiers::ALT, modifiers.alt_key());
    result.set(keyboard::Modifiers::LOGO, modifiers.super_key());
    result
}

/// Convert a Tao window event to an Iced event.
/// This is similar to winit's conversion::window_event() function.
pub fn convert_window_event(
    event: &TaoWindowEvent,
    scale_factor: f32,
    modifiers: keyboard::Modifiers,
) -> Option<Event> {

    match event {
        TaoWindowEvent::Resized(new_size) => {
            let logical_size = new_size.to_logical::<f64>(scale_factor.into());
            Some(Event::Window(iced_core::window::Event::Resized(Size {
                width: logical_size.width as f32,
                height: logical_size.height as f32,
            })))
        }
        TaoWindowEvent::CursorMoved { position, .. } => {
            let position = position.to_logical::<f64>(scale_factor.into());
            Some(Event::Mouse(mouse::Event::CursorMoved {
                position: Point::new(position.x as f32, position.y as f32),
            }))
        }
        TaoWindowEvent::MouseInput { button, state, .. } => {
            if let Some(mouse_button) = convert_mouse_button(button) {
                let mouse_event = match state {
                    ElementState::Pressed => mouse::Event::ButtonPressed(mouse_button),
                    ElementState::Released => mouse::Event::ButtonReleased(mouse_button),
                    _ => return None,
                };
                Some(Event::Mouse(mouse_event))
            } else {
                None
            }
        }
        TaoWindowEvent::ModifiersChanged(new_modifiers) => {
            log::debug!("ModifiersChanged - Tao modifiers: {:?}", new_modifiers);
            let iced_modifiers = convert_modifiers(new_modifiers);
            log::debug!("ModifiersChanged - Iced modifiers: {:?}", iced_modifiers);
            Some(Event::Keyboard(keyboard::Event::ModifiersChanged(
                iced_modifiers,
            )))
        }
        TaoWindowEvent::KeyboardInput { event, .. } => {
            let key = convert_key(&event.logical_key);
            let modified_key = convert_key(&event.logical_key);
            let physical_key = convert_physical_key(&event.physical_key);
            let location = convert_location(&event.location);

            log::debug!(
                "KeyboardInput - Tao key: {:?}, modifiers: {:?}",
                key,
                modifiers
            );

            let keyboard_event = match event.state {
                ElementState::Pressed => keyboard::Event::KeyPressed {
                    key,
                    modified_key,
                    physical_key,
                    location,
                    modifiers,
                    text: event.text.map(SmolStr::new),
                    repeat: event.repeat,
                },
                ElementState::Released => keyboard::Event::KeyReleased {
                    key,
                    modified_key,
                    physical_key,
                    location,
                    modifiers,
                },
                _ => return None,
            };

            log::debug!("KeyboardInput - Iced event: {:?}", keyboard_event);
            Some(Event::Keyboard(keyboard_event))
        }
        _ => {
            log::debug!("Unhandled event type: {:?}", std::mem::discriminant(event));
            None
        }
    }
}
