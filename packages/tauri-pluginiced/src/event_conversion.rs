// Event conversion module
// This will contain Tauri → Iced event mapping

use iced_core::mouse;
use iced_core::Size;
use iced_wgpu::graphics::Viewport;
use iced_winit::core::Point;
use tauri_runtime_wry::tao::event::MouseButton;
use tauri_runtime_wry::tao::keyboard::ModifiersState;

/// Convert physical mouse position to logical position with scale factor.
pub fn convert_mouse_position(
    physical_x: f64,
    physical_y: f64,
    scale_factor: f32,
) -> Option<Point> {
    Some(Point::new(
        physical_x as f32 / scale_factor,
        physical_y as f32 / scale_factor,
    ))
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

/// Convert Tauri modifier keys to Iced modifiers.
pub fn convert_modifiers(modifiers: &ModifiersState) -> ModifiersState {
    *modifiers
}

/// Create a viewport from physical dimensions and scale factor.
pub fn create_viewport(physical_width: u32, physical_height: u32, scale_factor: f32) -> Viewport {
    Viewport::with_physical_size(Size::new(physical_width, physical_height), scale_factor)
}
