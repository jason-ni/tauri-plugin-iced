use iced::widget::canvas;
use iced::widget::{button, column, stack, text, text_input};
use iced::{Color, Theme};
use tauri_plugin_iced::AppHandleExt;
use tauri_plugin_iced::IcedControls;

struct RandomWord {
    word: String,
}

impl RandomWord {
    const WORDS: [&'static str; 10] = [
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
        "kiwi",
        "lemon",
    ];

    fn new() -> Self {
        let word_index = rand::random::<usize>() % Self::WORDS.len();
        RandomWord {
            word: Self::WORDS[word_index].to_string(),
        }
    }
}

#[derive(Default)]
struct Counter {
    value: i32,
    text_input: String,
    window_counter: usize,
    app_handle: Option<tauri::AppHandle>,
}

#[derive(Debug, Clone)]
enum CounterMessage {
    Increment,
    Decrement,
    TextInputChanged(String),
    CreateRandomWindow,
}

impl IcedControls for Counter {
    type Message = CounterMessage;

    fn view(&self) -> iced::Element<'_, Self::Message, Theme, iced::Renderer> {
        stack![
            // Bottom layer: interactive widgets
            column![
                text("Counter: ").size(30),
                text(self.value).size(30),
                button("+").on_press(CounterMessage::Increment),
                button("-").on_press(CounterMessage::Decrement),
                text("Text Input Demo:").size(20),
                text_input("Type here...", &self.text_input)
                    .on_input(|s| CounterMessage::TextInputChanged(s))
                    .padding(10),
                button("Create Random Window").on_press(CounterMessage::CreateRandomWindow),
            ]
            .spacing(20)
            .padding(20),
            // Top layer: canvas overlay
            canvas(self)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
        ]
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
            CounterMessage::CreateRandomWindow => {
                if let Some(ref app_handle) = self.app_handle {
                    self.window_counter += 1;
                    let label = format!("window_{}", self.window_counter);

                    log::info!("Creating random word window: {}", label);

                    let wind = tauri::Window::builder(app_handle, &label)
                        .inner_size(400.0, 300.0)
                        .build()
                        .map_err(|e| format!("Failed to create window: {}", e));

                    match wind {
                        Ok(w) => {
                            let _ = w.show();

                            let random_word = RandomWord::new();

                            if let Err(e) =
                                app_handle.create_iced_window(&label, Box::new(random_word))
                            {
                                log::error!("Failed to create iced window: {}", e);
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to create window: {}", e);
                        }
                    }
                }
            }
        }
    }
}

impl IcedControls for RandomWord {
    type Message = CounterMessage;

    fn view(&self) -> iced::Element<'_, Self::Message, Theme, iced::Renderer> {
        canvas(self)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    fn update(&mut self, _message: Self::Message) {}
}

impl<Message> canvas::Program<Message> for RandomWord {
    type State = ();

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> iced::mouse::Interaction {
        iced::mouse::Interaction::None
    }

    fn draw(
        &self,
        _state: &(),
        render: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry<iced::Renderer>> {
        let mut frame = canvas::Frame::new(render, bounds.size());

        frame.fill_text(canvas::Text {
            position: frame.center(),
            content: self.word.clone(),
            color: Color::from_rgb8(255, 255, 255),
            size: 40.0.into(),
            ..canvas::Text::default()
        });

        vec![frame.into_geometry()]
    }
}

impl<Message> canvas::Program<Message> for Counter {
    type State = ();

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> iced::mouse::Interaction {
        iced::mouse::Interaction::None
    }

    fn draw(
        &self,
        _state: &(),
        render: &iced::Renderer,
        _theme: &Theme,
        bounds: iced::Rectangle,
        cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry<iced::Renderer>> {
        let mut frame = canvas::Frame::new(render, bounds.size());

        frame.fill_text(canvas::Text {
            position: iced::Point::new(frame.center().x, frame.center().y - 20.0),
            content: "hello world!".to_string(),
            color: Color::from_rgb8(0, 255, 255),
            size: 20.0.into(),
            ..canvas::Text::default()
        });

        if let Some(cursor_pos) = cursor.position() {
            let relative_pos = iced::Point::new(cursor_pos.x - bounds.x, cursor_pos.y - bounds.y);

            frame.fill_text(canvas::Text {
                position: relative_pos,
                content: format!("x: {:.0}, y: {:.0}", relative_pos.x, relative_pos.y),
                color: Color::WHITE,
                size: 14.0.into(),
                ..canvas::Text::default()
            });
        }

        vec![frame.into_geometry()]
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

            let mut counter = Counter::default();
            counter.app_handle = Some(app.handle().to_owned());
            app.handle().create_iced_window("main", Box::new(counter))?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
