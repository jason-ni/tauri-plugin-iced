pub mod event_conversion;
pub mod plugin;
pub mod renderer;
pub mod scene;
pub mod utils;

use iced::theme::Theme;
use iced_tiny_skia::Renderer;
use iced_winit::core::{Color, Element};

pub trait IcedControls: Send + Sync {
    type Message;

    fn view(&self) -> Element<'_, Self::Message, Theme, Renderer>;

    fn update(&mut self, message: Self::Message);

    fn background_color(&self) -> Color {
        Color::WHITE
    }
}

pub use event_conversion::{
    convert_location, convert_modifiers, convert_mouse_button, convert_mouse_position,
    convert_window_event, create_viewport,
};
pub use plugin::{AppHandleExt, Builder};
pub use scene::Scene;
pub use utils::IcedWindow;
