pub mod event_conversion;
pub mod plugin;
pub mod renderer;
pub mod scene;
pub mod utils;

use iced::{theme::Theme, Element};
use iced_wgpu::Renderer;
use iced_winit::core::Color;

pub trait IcedControls: Send + Sync {
    type Message;

    fn view(&self) -> Element<'_, Self::Message, Theme, Renderer>;

    fn update(&mut self, message: Self::Message);

    fn handle_event(&mut self, _event: &iced_winit::core::Event) {}

    fn background_color(&self) -> Color {
        Color::BLACK
    }
}

pub use event_conversion::{
    convert_modifiers, convert_mouse_button, convert_mouse_position, create_viewport,
};
pub use plugin::{AppHandleExt, Builder};
pub use scene::Scene;
pub use utils::IcedWindow;
