# tauri-plugin-iced

A Tauri plugin for integrating the [Iced](https://github.com/iced-rs/iced) GUI framework with Tauri applications. This plugin enables rendering native Iced UI components in Tauri-managed windows using GPU-accelerated rendering.

## Features

- **Native Iced rendering**: Render Iced's retained-mode UI in Tauri windows
- **GPU acceleration**: Uses WGPU for hardware-accelerated rendering
- **Multiple windows**: Support for multiple independent Iced windows with separate state
- **Flexible UI types**: Each window can use different concrete IcedControls implementations
- **Event handling**: Automatic conversion of Tauri events to Iced events
- **Thread-safe**: Uses `Arc<Mutex<>>` for concurrent window access
- **Cursor management**: Automatic cursor icon updates based on UI state

## Installation

Add this plugin to your `Cargo.toml`:

```toml
[dependencies]
tauri = { version = "2.7.0", features = ["unstable"] }
tauri-plugin-iced = { path = "packages/tauri-pluginiced" }
iced = { version = "0.13", features = ["wgpu"] }
```

## Quick Start

### 1. Implement the IcedControls Trait

Define your UI state and behavior by implementing the `IcedControls` trait.

Define a custom Message type for your UI events:

```rust
#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl IcedControls for MyControls {
    type Message = Message;

```rust
#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl IcedControls for MyControls {
    type Message = Message;

    fn view(&self) -> iced::Element<Self::Message> {
        column![
            text("Count: ").size(30),
            text(self.count).size(30),
            button("+").on_press(Message::Increment),
            button("-").on_press(Message::Decrement),
        ]
        .spacing(20)
        .padding(20)
        .into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
    }
}
```

### 2. Initialize the Plugin

Register the plugin with your Tauri app:

```rust
use tauri_plugin_iced::Builder;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize the plugin with your Message type
            let plugin = Builder::<()>::new(app.handle().to_owned());
            app.wry_plugin(plugin);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

If you're using a custom Message type:

```rust
use tauri_plugin_iced::Builder;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let plugin = Builder::<MyMessage>::new(app.handle().to_owned());
            app.wry_plugin(plugin);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3. Create an Iced Window

Attach Iced rendering to a Tauri window:

```rust
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // ... plugin initialization ...

            // Create a Tauri window
            let window = tauri::Window::builder(app, "main")
                .title("My Iced Window")
                .build()?;

            // Attach Iced rendering to the window
            app.handle().create_iced_window("main", Box::new(MyControls::default()))?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

You can create multiple windows with different control types:

```rust
app.handle().create_iced_window("main", Box::new(Counter::default()))?;
app.handle().create_iced_window("settings", Box::new(Settings::default()))?;
```

## API Reference

### IcedControls Trait

The core trait for defining Iced UI behavior.

```rust
pub trait IcedControls: Send + Sync {
    type Message;

    fn view(&self) -> Element<Self::Message>;
    fn update(&mut self, message: Self::Message);
    fn background_color(&self) -> Color { Color::BLACK }
}
```

- `type Message`: The enum type for UI events
- `view(&self)`: Build UI from current state
- `update(&mut self, message)`: Handle state changes
- `background_color(&self)`: (Optional) Set background color for the window

### create_iced_window()

Attaches Iced rendering to an existing Tauri window.

```rust
app.handle().create_iced_window(
    label: &str,
    controls: Box<dyn IcedControls<Message = M> + Send + Sync>,
) -> Result<(), Error>
```

**Parameters:**
- `label`: The identifier of the Tauri window to attach Iced to
- `controls`: Boxed instance of your `IcedControls` implementation (must use `Send + Sync`)

**Returns:** `Result<(), Error>` - Success or error if window not found

**Note:** The `Message` type must match the `M` type used in `Builder<M>`.

### Builder

Builder for creating the plugin instance.

```rust
let plugin = Builder::<M>::new(app_handle);
app.wry_plugin(plugin);
```

**Type Parameter:**
- `M`: The Message type shared across all Iced windows

**Note:** The builder does not require calling `.build()` - pass it directly to `app.wry_plugin()`.

### create_iced_window()

Attaches Iced rendering to an existing Tauri window.

```rust
app.handle().create_iced_window(
    label: &str,           // Window label
    controls: Box<dyn IcedControls>  // Your UI implementation
) -> Result<(), Error>
```

**Parameters:**
- `label`: The identifier of the Tauri window to attach Iced to
- `controls`: Boxed instance of your `IcedControls` implementation

**Returns:** `Result<(), Error>` - Success or error if window not found

### Builder

Builder for creating the plugin instance.

```rust
let plugin = Builder::new(app_handle)
    .build()?;
```

## Event Handling

The plugin automatically converts Tauri window events to Iced events:

- **Mouse events**: Position tracking, button clicks, scrolling
- **Keyboard events**: Key presses, modifier keys (shift, ctrl, alt, cmd)
- **Window events**: Resize handling, viewport updates

Events are accumulated during the window event phase and processed in batches for efficiency.

## Clipboard

The plugin uses a headless clipboard implementation (`Clipboard::unconnected()`). This provides basic clipboard functionality but may have limitations compared to a fully integrated system clipboard.

Future improvements may investigate direct winit window access for full clipboard integration.

## Thread Safety

The plugin uses `Arc<Mutex<HashMap<String, IcedWindow>>>` to manage windows in a thread-safe manner. All window accesses are protected by mutex locks with minimal scope to prevent deadlocks.

The design follows the pattern established by [tauri-plugin-egui](https://github.com/tauri-apps/tauri-plugin-egui), ensuring proven thread safety.

## Platform Support

- **macOS**: ✓
- **Linux**: ✓
- **Windows**: ✓

Cross-platform testing is ongoing. Please report any platform-specific issues.

## Limitations

- No async Commands/Subscriptions (synchronous message processing only)
- Clipboard integration uses headless fallback
- IME (Input Method Editor) support not yet implemented
- Web/WASM support not yet implemented (desktop-only)

## Examples

See `packages/test_app` for a complete example application demonstrating:
- Counter UI with increment/decrement buttons
- Plugin initialization
- Window creation and Iced attachment

## Architecture

The plugin follows Iced's headless rendering pattern:

1. Tauri owns the window lifecycle and event loop
2. Plugin intercepts Tauri events via `wry_plugin` mechanism
3. Events are converted to Iced events and fed to the UI
4. Iced renders to a WGPU surface managed by the plugin
5. Rendered frames are presented in the Tauri window

## Credits

Inspired by [tauri-plugin-egui](https://github.com/tauri-apps/tauri-plugin-egui) and the Iced integration examples.

## License

MIT OR Apache-2.0
