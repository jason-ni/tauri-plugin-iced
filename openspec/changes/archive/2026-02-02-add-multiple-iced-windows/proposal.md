## Why

To test whether the tauri-plugin-iced can create and manage multiple independent iced windows simultaneously, demonstrating the plugin's multi-window capabilities and providing a foundation for future multi-window applications.

## What Changes

- Add `CreateRandomWindow` message variant to `CounterMessage` enum
- Add `window_counter: usize` field to `Counter` struct for unique window label generation
- Add `app_handle: Option<tauri::AppHandle>` field to `Counter` struct for direct window creation
- Update `Counter` initialization to provide AppHandle during setup
- Add "Create Random Window" button to main Counter UI (below text input)
- Implement `Counter::update()` handler for `CreateRandomWindow` message that:
  - Increments window counter
  - Generates unique window label (e.g., `window_1`, `window_2`)
  - Creates new Tauri window with 400x300 dimensions using AppHandle
  - Creates new `RandomWord` instance with randomly selected word
  - Attaches iced renderer via `app_handle.create_iced_window()`
- Create new `RandomWord` struct implementing `IcedControls` trait:
  - Static 10-word list: ["apple", "banana", "cherry", "date", "elderberry", "fig", "grape", "honeydew", "kiwi", "lemon"]
  - `word: String` field storing the randomly selected word
  - `view()` returning canvas with word centered in window
  - `update()` as no-op (static content, ignores all messages)
  - `new()` constructor that picks random word from list using `rand` crate
- Add `rand` dependency to test_app `Cargo.toml`
- Modify plugin to support multiple windows with shared `Message` type (already supported via `HashMap<String, IcedWindow<M>>`)

## Capabilities

### New Capabilities
- `multiple-iced-windows`: Creating and managing multiple independent iced windows with shared message type, each having its own state and rendering content

### Modified Capabilities
- (none - no existing spec requirements are changing)

## Impact

- Changes to `packages/test_app/src-tauri/src/lib.rs` to add `RandomWord` struct and update `Counter` struct
- New dependency: `rand` crate added to `packages/test_app/src-tauri/Cargo.toml`
- No API changes to tauri-plugin-iced plugin (already supports multiple windows)
- No breaking changes to existing functionality
