## Context

The tauri-plugin-iced currently supports a single iced window instance via `create_iced_window()`. The plugin uses a `HashMap<String, IcedWindow<M>>` to store windows, suggesting multi-window support is architecturally possible. The existing test app demonstrates basic iced functionality with a counter and canvas overlay. This change tests the plugin's multi-window capabilities by allowing users to dynamically create new windows, each displaying a random word in a canvas.

**Current state:**
- Plugin manages windows via `HashMap<String, IcedWindow<M>>`
- Test app has one main window with Counter state
- All windows share the same generic `Message` type parameter `<M>`

**Constraints:**
- All windows must share the same `Message` type (CounterMessage)
- RandomWord windows are static content (ignore all messages in update())

## Goals / Non-Goals

**Goals:**
- Demonstrate multi-window support in tauri-plugin-iced
- Create a pattern for dynamically adding new windows from existing iced UI
- Validate that each window maintains independent state
- Show canvas rendering in multiple windows simultaneously

**Non-Goals:**
- Cross-window communication or message passing
- Window persistence or state management across restarts
- Complex random word selection or animation
- Window positioning or layout management

## Decisions

### Message Type Design: Shared enum across all windows

**Decision:** All windows share `CounterMessage` enum, with RandomWord windows ignoring all messages in `update()`.

**Rationale:**
- Plugin's generic `<M>` type is set once for all windows in the HashMap
- Adding a new message variant is simpler than implementing a trait object or union type
- RandomWord windows are static, so ignoring messages is acceptable
- Follows the pattern hinted in lib.rs:174 (Message = () and override handle_event)

**Alternatives considered:**
1. **Per-window message types with unified wrapper enum:**
   - Would require `CounterMessage` to be a wrapper around `CounterMessage` and `RandomWordMessage`
   - Adds complexity without benefit for this use case
   - Would require major refactoring of plugin's generic constraints

2. **Use Message = () everywhere and handle via Tauri commands:**
   - Would lose the ability to trigger window creation from iced button press
   - Breaks the IcedControls update() pattern

### Window Creation: Direct access via AppHandle in Counter

**Decision:** Add `app_handle: Option<tauri::AppHandle>` field to Counter struct, enabling direct window creation in `update()` method.

**Rationale:**
- Simplifies architecture - no need for separate Tauri commands or message routing
- Counter can directly call `app_handle.create_iced_window()` when handling `CreateRandomWindow` message
- AppHandle is provided during initial Counter setup (before first window creation)
- More idiomatic Rust pattern for this use case

**Alternatives considered:**
1. **Tauri command with external trigger:**
   - Would require complex message routing from Iced to Tauri
   - Breaks the IcedControls update() pattern
   - More complex implementation for simple feature

2. **Callback or trait injection:**
   - Over-engineering for this demonstration
   - AppHandle is already the natural way to access Tauri APIs

### Window Label Generation: Simple counter in Counter struct

**Decision:** Store `window_counter: usize` in Counter, generate labels as `window_{N}`.

**Rationale:**
- Simple, predictable, guarantees uniqueness
- Counter state is preserved across button clicks
- Labels follow Tauri's naming conventions
- No external dependencies or complex state management

**Alternatives considered:**
1. **UUID generation:**
   - More complex, unnecessary for this use case
   - Harder to debug/logs readability

2. **Time-based labels:**
   - Could theoretically collide in rapid clicks
   - Less predictable for debugging

### Window Size: Fixed 400x300 for RandomWord windows

**Decision:** All RandomWord windows use 400x300 dimensions.

**Rationale:**
- Simpler than variable or user-configurable sizing
- Distinguishes RandomWord windows from main window visually
- Large enough to display word clearly with good padding

### Random Word Selection: Static list with random index

**Decision:** Use fixed 10-word list and pick random index on window creation.

**Rationale:**
- Deterministic word set makes testing easier
- Simple implementation using `rand` crate's `random::<usize>() % WORDS.len()`
- Words cycle naturally as more windows are created (or random repeats)
- No external file I/O or network calls

**Alternatives considered:**
1. **Sequential cycling through words:**
   - Less interesting visually (predictable pattern)
   - User asked for "random words"

2. **External word list file:**
   - Adds complexity without benefit
   - Test app doesn't need extensible word lists

### RandomWord as Static Content: No-op update() method

**Decision:** `RandomWord::update()` does nothing for all messages.

**Rationale:**
- Word never changes after creation
- Simplifies implementation
- No need for additional message handling logic
- Performance benefit (no unnecessary re-renders)

## Risks / Trade-offs

### Risk: Plugin may not handle multiple windows correctly

**Mitigation:**
- Plugin already uses `HashMap<String, IcedWindow<M>>` architecture
- Event routing already handles window ID mapping
- Test with creating 10+ windows to validate scaling

### Risk: Memory leak if windows are not properly cleaned up

**Mitigation:**
- Plugin already handles `Destroyed` events (plugin.rs:247-254)
- HashMap cleanup happens when Tauri window closes
- Verify with repeated create/close cycles

### Risk: Window creation race condition

**Mitigation:**
- Plugin uses `StagingWindowWrapper` to handle race during creation (plugin.rs:26-28)
- `transfer_staging_window()` ensures windows are added before first render
- Follow existing pattern used for initial main window

### Trade-off: Static word list limits variety

**Impact:**
- Only 10 unique words available
- Windows may repeat words before all 10 are shown
- Acceptable for demonstration purposes

## Migration Plan

This is additive functionality only. No migration steps required.

**Testing approach:**
1. Start test app, verify main window works as before
2. Click "Create Random Window" button 3-4 times
3. Verify each window appears with different/random word
4. Verify main window continues to work (counter, buttons, text input)
5. Close random windows one by one, verify no crashes
6. Create more windows, verify no performance degradation

## Open Questions

None. Design is straightforward based on exploration.
