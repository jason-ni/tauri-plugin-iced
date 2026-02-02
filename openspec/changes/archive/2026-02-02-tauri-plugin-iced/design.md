## Context

This project aims to integrate Iced's retained-mode GUI system with Tauri applications. The primary constraint is that Tauri owns the window lifecycle and event loop, so Iced must work as a "headless" renderer that renders into Tauri-created windows.

The design follows the pattern established by `tauri-plugin-egui`, which successfully demonstrates how to integrate an immediate-mode GUI (egui) with Tauri. However, Iced's retained-mode architecture (ELM pattern: Model-View-Update) requires a different approach to state management compared to egui's closure-based immediate mode.

Key constraints:
- Tauri v2.7.0+ required for `wry_plugin` API
- Iced uses a headless shell renderer to avoid owning the event loop
- No async runtime for message processing (synchronous in event loop)
- Must support multiple independent windows with separate state

## Goals / Non-Goals

**Goals:**
- Enable rendering Iced UI in Tauri-managed native windows
- Provide a clean trait-based API for users to define UI behavior
- Support multiple concurrent Iced windows with independent state
- Achieve native GPU rendering performance via WGPU
- Follow established patterns from tauri-plugin-egui where applicable

**Non-Goals:**
- Commands and subscriptions (async work) - deferred to future work
- IME (Input Method Editor) support initially
- Web/wasm support (desktop-only initially)
- Creating windows internally - only accepting existing Tauri windows

## Decisions

### 1. Follow tauri-plugin-egui Architecture Pattern

**Decision:** Adopt the plugin structure from tauri-plugin-egui as the baseline architecture.

**Rationale:**
- Proven pattern for integrating GUI frameworks with Tauri
- Demonstrates correct usage of `wry_plugin` mechanism
- Shows thread-safe window management with `Arc<Mutex<HashMap<String, IcedWindow>>>`
- Reduces risk by following established working code

**Alternatives considered:**
- Custom architecture from scratch: Higher risk, more unknowns
- Fork Tauri event loop: Would require Tauri core changes (rejected)

### 2. IcedControls Trait with Associated Message Type

**Decision:** Use associated type for `Message` in `IcedControls` trait.

```rust
pub trait IcedControls {
    type Message;
    
    fn view(&self) -> Element<Self::Message>;
    fn update(&mut self, message: Self::Message);
}
```

**Rationale:**
- Each UI has its own message type (no shared enum across all windows)
- Clean type safety - compiler ensures message handling matches
- Consistent with Iced's Application pattern
- Simpler than generic trait object over Message

**Alternatives considered:**
- Generic trait object `Box<dyn IcedControls<Message>>`: Would require boxing with specific Message type, more complex API
- Single shared message enum: Doesn't scale with multiple independent windows

### 3. Headless Iced Renderer (Shell::headless())

**Decision:** Use `iced_wgpu::Engine::new(..., Shell::headless(), ...)` to initialize Iced without owning the window.

**Rationale:**
- Iced's default expects to own the winit event loop
- Headless shell allows external event source (Tauri)
- Pattern demonstrated in `refer/iced/examples/integration`
- Clean separation of concerns (Tauri handles events, Iced handles rendering)

**Alternatives considered:**
- Iced's default Application trait: Would require owning event loop (incompatible with Tauri)
- Custom Iced fork: High maintenance burden

### 4. Synchronous Message Processing (No Commands/Subscriptions)

**Decision:** Process UI messages synchronously in the event loop iteration, ignoring Iced's `Command` and `Subscription` types.

**Rationale:**
- tauri-plugin-egui demonstrates this works well
- Simpler initial implementation
- No async runtime complexity
- Commands/Subscriptions can be added later if needed

**Alternatives considered:**
- Execute Commands via Tauri async_runtime: Adds complexity, not needed for initial release

### 5. Event Accumulation Pattern

**Decision:** Accumulate events in a `Vec<Event>` during window event phase, then process all at once when non-empty.

**Rationale:**
- Pattern from integration example (lines 334-365 of main.rs)
- Efficient - batches event processing
- Iced's `UserInterface::update()` expects slice of events
- Natural fit for event-driven architecture

**Flow:**
```
WindowEvent phase:
  - Convert Tauri event → Iced event
  - Push to events vector
  - If events non-empty:
    - Build UI from controls.view()
    - Update UI with events (collects messages)
    - Call controls.update() for each message
    - Request redraw

RedrawRequested phase:
  - Build UI from controls.view()
  - Process RedrawRequested event
  - Draw to WGPU surface
  - Present frame
```

### 6. WGPU Rendering Abstraction

**Decision:** Create a `Renderer` struct that wraps WGPU initialization and surface management, similar to tauri-plugin-egui.

**Rationale:**
- Encapsulates WGPU complexity
- Handles surface configuration, device/queue management
- Provides clean API: `new()`, `resize()`, `render_frame()`
- Matches pattern from integration example

**Components:**
```rust
struct Renderer {
    gpu: Gpu,  // Surface, device, queue, config
    renderer: iced_wgpu::Renderer,
}

struct Gpu {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    surface_format: wgpu::TextureFormat,
}
```

### 7. Clipboard Integration via Window Handle

**Decision:** Pass a Tauri window handle to create Iced `Clipboard` instance.

**Rationale:**
- Iced's `Clipboard::connect()` expects a winit window
- Tauri's `Window` wraps the underlying winit/tao window
- Need to investigate how to access the raw window handle

**Open question:** How to get winit window from Tauri Window?
- May need to access internal wry window via private APIs
- Alternative: Create headless clipboard (may have limited functionality)

### 8. Event Conversion Module

**Decision:** Create dedicated `event_conversion.rs` module to handle Tauri → Iced event mapping.

**Rationale:**
- Clean separation of concerns
- Reusable mapping logic
- Similar to iced_winit's `conversion` module

**Mappings required:**
- Mouse position: Tauri (physical) → Iced (logical with scale factor)
- Mouse buttons: Tauri MouseButton → Iced PointerButton
- Keyboard: Tauri KeyEvent → Iced Key event (including modifiers)
- Window events: Resize → viewport update

### 9. Cursor State Tracking

**Decision:** Track cursor position and modifiers state in `IcedWindow`, convert mouse interaction to cursor icon.

**Rationale:**
- Iced reports `mouse_interaction` after processing events
- Need to convert to Tauri's `CursorIcon` enum
- Matches integration example pattern (lines 273-286)

**Flow:**
```
After UI update:
  - Check if mouse_interaction state updated
  - If yes, convert to Tauri cursor icon
  - Send SetCursorIcon message to window
```

### 10. Staging Pattern for Window Registration

**Decision:** Use a "staging window" pattern similar to tauri-plugin-egui to handle race conditions during window creation.

**Rationale:**
- Plugin receives events before user calls `create_iced_window()`
- Staging window holds pending IcedWindow until events arrive
- Transfers to main hashmap on first event matching the label
- Pattern proven in tauri-plugin-egui (lines 145-157 of plugin.rs)

## Risks / Trade-offs

### Clipboard Integration Risk

**Risk:** Cannot access underlying winit window from Tauri's `Window` type to create Iced `Clipboard`.

**Mitigation:**
- Investigate Tauri's internal wry window access patterns
- May need to use `unsafe` to access private field as last resort
- Alternative: Use headless clipboard (limited but functional)

### Thread Safety Complexity

**Risk:** `Arc<Mutex<>>` adds complexity and potential for deadlocks if not careful.

**Mitigation:**
- Keep lock scope minimal
- Never hold lock across async boundaries
- Follow tauri-plugin-egui's proven patterns
- Consider using dashmap for better performance (future optimization)

### Iced Version Compatibility

**Risk:** Iced is still evolving rapidly; API changes may break integration.

**Mitigation:**
- Pin to specific Iced version in Cargo.toml
- Monitor Iced release notes for breaking changes
- Consider using Iced's workspace dependencies if possible

### Performance Overhead

**Risk:** Mutex locking on every event could impact performance.

**Mitigation:**
- Benchmark with tauri-plugin-egui as baseline
- Consider lock-free patterns if bottleneck identified
- Only lock when accessing window state, not for event conversion

### Platform-Specific Behavior

**Risk:** Different platforms (Windows, macOS, Linux) may have subtle differences in event handling.

**Mitigation:**
- Test on all target platforms
- Use Iced's cross-platform abstractions where possible
- Document any platform-specific workarounds

## Migration Plan

This is a new capability - no migration required for existing code.

Deployment steps:
1. Create workspace structure with `Cargo.toml`
2. Create `packages/tauri-plugin-iced` package
3. Implement core components (trait, renderer, plugin)
4. Create `packages/test_app` to demonstrate usage
5. Add documentation and examples

Rollback strategy:
- Delete plugin packages if integration proves problematic
- No breaking changes to existing code

## Open Questions

1. **Clipboard access**: How to access the underlying winit window from Tauri's `Window` type?
   - Options: Private field access, reflection, headless clipboard
   - Priority: High - needed for initial implementation

2. **Iced workspace dependencies**: Should we use Iced's workspace or pin specific versions?
   - Iced recommends workspace dependencies for consistency
   - Need to test workspace compatibility

3. **Cursor icon mapping**: Are all Iced cursor icons supported by Tauri?
   - May need fallback mapping for unsupported icons

4. **Input handling**: Should we use `rdev` like tauri-plugin-egui for global mouse events?
   - Integration example doesn't use it
   - Defer to future if scroll issues arise

5. **Theme support**: Should we expose theme selection to users?
   - Integration example hardcodes `Theme::Dark`
   - Could make `IcedControls` have optional `theme()` method
