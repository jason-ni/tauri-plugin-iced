use iced::widget::canvas;
use iced::widget::{button, column, container, stack, text, text_input};
use iced::{Color, ContentFit, Length, Theme};
use tauri_plugin_iced::AppHandleExt;
use tauri_plugin_iced::IcedControls;

struct ScreenshotData {
    handle: iced::widget::image::Handle,
    width: u32,
    height: u32,
}

enum ViewerContent {
    Screenshot(iced::widget::image::Handle),
    Error(String),
}

struct ScreenshotViewer {
    content: ViewerContent,
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
    CaptureScreenshot,
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
                button("Take Screenshot").on_press(CounterMessage::CaptureScreenshot),
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
            CounterMessage::CaptureScreenshot => {
                if let Some(ref app_handle) = self.app_handle {
                    let result = capture_primary_screen();

                    self.window_counter += 1;
                    let label = format!("screenshot_{}", self.window_counter);

                    log::info!("Creating screenshot window: {}", label);

                    let (window_width, window_height) = match &result {
                        Ok(data) => calculate_window_size(data.width, data.height),
                        Err(_) => (800.0, 600.0),
                    };

                    let wind = tauri::Window::builder(app_handle, &label)
                        .inner_size(window_width, window_height)
                        .build()
                        .map_err(|e| format!("Failed to create window: {}", e));

                    match wind {
                        Ok(w) => {
                            let _ = w.show();

                            let screenshot_viewer = ScreenshotViewer::new(result);

                            if let Err(e) =
                                app_handle.create_iced_window(&label, Box::new(screenshot_viewer))
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

impl ScreenshotViewer {
    fn new(result: Result<ScreenshotData, String>) -> Self {
        let content = match result {
            Ok(data) => {
                log::info!(
                    "Created ScreenshotViewer with screenshot: {}x{}",
                    data.width,
                    data.height
                );
                ViewerContent::Screenshot(data.handle)
            }
            Err(msg) => {
                log::info!("Created ScreenshotViewer with error: {}", msg);
                ViewerContent::Error(msg)
            }
        };
        ScreenshotViewer { content }
    }
}

impl IcedControls for ScreenshotViewer {
    type Message = CounterMessage;

    fn view(&self) -> iced::Element<'_, Self::Message, Theme, iced::Renderer> {
        use iced::widget::image;

        let element = match &self.content {
            ViewerContent::Screenshot(handle) => {
                container(image(handle).content_fit(ContentFit::Contain))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
            }
            ViewerContent::Error(msg) => container(text(format!("Capture failed: {}", msg)))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into(),
        };

        element
    }

    fn update(&mut self, _message: Self::Message) {}
}

fn capture_primary_screen() -> Result<ScreenshotData, String> {
    use xcap::Monitor;

    let monitors = Monitor::all().map_err(|e| format!("Failed to get screens: {}", e))?;

    if monitors.is_empty() {
        return Err("No screens found".to_string());
    }

    let monitor = &monitors[0];
    let rgba_image = monitor
        .capture_image()
        .map_err(|e| format!("Failed to capture screen: {}", e))?;

    let width = rgba_image.width();
    let height = rgba_image.height();
    let rgba_vec = rgba_image.into_raw();

    log::info!(
        "Captured screen: {}x{}, {} bytes",
        width,
        height,
        rgba_vec.len()
    );

    let handle = iced::widget::image::Handle::from_rgba(width, height, rgba_vec);

    log::info!("Created image handle directly from xcap data");

    Ok(ScreenshotData {
        handle,
        width,
        height,
    })
}

fn calculate_window_size(screen_w: u32, screen_h: u32) -> (f64, f64) {
    const MAX_WIDTH: f64 = 1200.0;
    const MAX_HEIGHT: f64 = 800.0;

    let scale = (MAX_WIDTH / screen_w as f64)
        .min(MAX_HEIGHT / screen_h as f64)
        .min(1.0);

    (screen_w as f64 * scale, screen_h as f64 * scale)
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
