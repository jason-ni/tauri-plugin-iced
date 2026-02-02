## Why

Tauri applications that need native UI performance or complex widgets beyond web technologies can use Iced's retained-mode GUI system. By creating a Tauri plugin for Iced, developers can render native Iced UI in Tauri-managed windows, following the pattern established by tauri-plugin-egui.

## What Changes

- Create new Rust workspace structure with workspace root `Cargo.toml`
- Create `tauri-plugin-iced` package as a Tauri plugin
- Create `test_app` Tauri application to demonstrate and test the plugin
- Implement `IcedControls` trait for users to define their UI behavior
- Implement WGPU-based rendering backend using Iced's headless renderer
- Implement Tauri event loop integration via `wry_plugin` mechanism
- Implement event conversion from Tauri window events to Iced events
- Implement window lifecycle management using `Arc<Mutex<HashMap<String, IcedWindow>>>`
- Implement clipboard integration with Iced's Clipboard API

## Capabilities

### New Capabilities
- `tauri-iced-window`: Plugin for creating and managing Iced-rendered native windows in Tauri applications

### Modified Capabilities
- None

## Impact

- Adds `iced` and related dependencies (iced_wgpu, iced_winit, iced_widget) to the workspace
- Adds `tauri` runtime dependencies (tauri-runtime, tauri-runtime-wry)
- Adds `wgpu` graphics dependency
- Requires Tauri v2.7.0+ (wry_plugin API)
- Creates new public API: `IcedControls` trait and `create_iced_window` method on AppHandle
