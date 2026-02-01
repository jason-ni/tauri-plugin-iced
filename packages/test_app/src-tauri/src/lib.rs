use iced::widget::{button, column, text, text_input};
use iced::Theme;
use iced_wgpu::Renderer;
use tauri_plugin_iced::AppHandleExt;
use tauri_plugin_iced::IcedControls;

#[derive(Default)]
struct Counter {
    value: i32,
    text_input: String,
}

#[derive(Debug, Clone)]
enum CounterMessage {
    Increment,
    Decrement,
    TextInputChanged(String),
}

impl IcedControls for Counter {
    type Message = CounterMessage;

    fn view(&self) -> iced::Element<'_, Self::Message, Theme, Renderer> {
        column![
            text("Counter: ").size(30),
            text(self.value).size(30),
            button("+").on_press(CounterMessage::Increment),
            button("-").on_press(CounterMessage::Decrement),
            text("Text Input Demo:").size(20),
            text_input("Type here...", &self.text_input)
                .on_input(|s| CounterMessage::TextInputChanged(s))
                .padding(10),
        ]
        .spacing(20)
        .padding(20)
        .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            CounterMessage::Increment => self.value += 1,
            CounterMessage::Decrement => self.value -= 1,
            CounterMessage::TextInputChanged(text) => {
                self.text_input = text;
                log::info!("Text input changed: {}", self.text_input);
            }
        }
    }
}

#[tauri::command]
fn create_iced_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    let wind = tauri::Window::builder(&app_handle, "iced_window")
        .build()
        .map_err(|e| format!("Failed to create window: {}", e))?;
    let _ = wind.show();

    log::info!("Created iced window");

    app_handle
        .create_iced_window("iced_window", Box::new(Counter::default()))
        .map_err(|e| format!("Failed to create iced window: {}", e))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_iced_window])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            log::info!("Iced Tauri Example started");
            let plugin = tauri_plugin_iced::Builder::<CounterMessage>::new(app.handle().to_owned());
            app.wry_plugin(plugin);

            let wind = tauri::Window::builder(app, "main")
                .build()
                .map_err(|e| format!("Failed to create window: {}", e))?;
            let _ = wind.show();

            app.handle()
                .create_iced_window("main", Box::new(Counter::default()))?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
