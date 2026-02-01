// Placeholder for tauri-plugin-iced library
pub mod plugin;
pub mod renderer;
pub mod event_conversion;
pub mod utils;

use iced::{
    theme::Theme,
    Element
};
use iced_wgpu::Renderer;

/// Trait that users implement to define their Iced UI behavior.
///
/// This follows as ELM (Model-View-Update) pattern where:
/// - View is defined by as `view()` method
/// - Updates are handled by as `update()` method
/// - Messages trigger state changes
///
/// For use with the Tauri plugin, use `Message = ()` and override
/// `handle_event` instead of `update`.
pub trait IcedControls: Send + Sync {
    /// The message type for this application.
    ///
    /// Each implementation can define its own message type,
    /// allowing for type-safe event handling.
    type Message;

    /// Returns the UI tree for rendering.
    ///
    /// This method is called whenever the UI needs to be redrawn.
    fn view(&self) -> Element<'_, Self::Message, Theme, Renderer>;

    /// Updates internal state based on a message.
    ///
    /// This method is called synchronously when a UI event generates a message.
    /// The message represents user interaction or other UI events.
    fn update(&mut self, message: Self::Message);

    /// Handle an Iced event directly (for trait object usage).
    ///
    /// This method should be implemented when using this trait with `Message = ()`.
    /// Default implementation does nothing.
    fn handle_event(&mut self, _event: &iced_winit::core::Event) {
    }
}

// Re-export public API from plugin module
pub use plugin::{Builder, AppHandleExt};
pub use event_conversion::{convert_mouse_position, convert_mouse_button, convert_modifiers, create_viewport};
pub use utils::IcedWindow;
