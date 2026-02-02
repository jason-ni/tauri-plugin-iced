## 1. Dependencies

- [x] 1.1 Add `rand` dependency to `packages/test_app/src-tauri/Cargo.toml`

## 2. Counter Struct Updates

- [x] 2.1 Add `window_counter: usize` field to `Counter` struct
- [x] 2.2 Add `app_handle: Option<tauri::AppHandle>` field to `Counter` struct
- [x] 2.3 Add `CreateRandomWindow` variant to `CounterMessage` enum
- [x] 2.4 Implement `CreateRandomWindow` handler in `Counter::update()` that:
  - Increments `window_counter`
  - Generates window label as `format!("window_{}", window_counter)`
  - Uses `app_handle` to create new Tauri window with 400x300 dimensions
  - Creates new `RandomWord` instance with random word
  - Calls `app_handle.create_iced_window(label, Box::new(random_word))`

## 3. RandomWord Struct

- [x] 3.1 Create `RandomWord` struct with:
  - `word: String` field
  - Static `WORDS: [&str; 10]` array with words
  - `new()` constructor that picks random word using `rand`
- [x] 3.2 Implement `IcedControls::view()` for `RandomWord` that returns canvas with word centered
- [x] 3.3 Implement `IcedControls::update()` for `RandomWord` as no-op (ignores all messages)
- [x] 3.4 Implement `canvas::Program` trait for `RandomWord` with:
  - `mouse_interaction()` returning `Interaction::None`
  - `draw()` method rendering word centered in canvas

## 4. UI Changes

- [x] 4.1 Add "Create Random Window" button to `Counter::view()` (below text input)
- [x] 4.2 Wire button to `CounterMessage::CreateRandomWindow`

## 5. Window Creation Logic

- [x] 5.1 Update `Counter::default()` to initialize `app_handle` as `None`
- [x] 5.2 Update setup in `run()` to provide AppHandle to initial Counter instance

## 6. Testing and Validation

- [x] 6.1 Build test app and verify no compilation errors
- [x] 6.2 Run test app and verify main window works as before
- [x] 6.3 Test creating 3-4 random word windows and verify:
  - Each window appears with random word
  - Words are centered in canvas
  - Windows have unique labels (check logs)
- [x] 6.4 Test that main window continues to work (counter, buttons, text input) while other windows exist
- [x] 6.5 Test closing random windows one by one and verify no crashes
- [x] 6.6 Test creating 10+ windows to validate scaling
- [x] 6.7 Verify no memory leaks after repeated create/close cycles
