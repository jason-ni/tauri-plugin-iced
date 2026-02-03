## Context

Current architecture creates a complete wgpu stack (instance, adapter, device, queue, surface) for each window. When windows close, these resources are never properly cleaned up, causing observable memory growth in applications that frequently create and destroy windows. The plugin uses `Arc<Mutex<HashMap<String, IcedWindow>>>` for window management, and all rendering occurs on the main event loop thread (single-threaded, synchronized by the windows Mutex).

## Goals / Non-Goals

**Goals:**
- Eliminate memory leak from repeated window creation/destruction by sharing GPU resources across all windows
- Maintain thread safety with single-threaded rendering model
- Keep user-facing API unchanged (plugin usage remains the same)
- Keep implementation simple and maintainable

**Non-Goals:**
- Implementing cleanup of GPU resources when all windows close (keep alive for now, optimize later if needed)
- Handling GPU context loss or adapter switching scenarios
- Supporting multi-threaded rendering (current design assumes single-threaded event loop)
- Optimizing for resource-constrained environments

## Decisions

### 1. Store shared resources in IcedPlugin as Option members

Store wgpu `instance`, `adapter`, `device`, and `queue` as `Option` fields in `IcedPlugin` rather than using `OnceLock` or wrapping in `Arc`. Since rendering is single-threaded and already synchronized by the `windows` Mutex, these simpler types are sufficient and avoid unnecessary complexity.

**Rationale:**
- Single-threaded access pattern means we don't need lock-free primitives (`OnceLock`)
- We already have exclusive access via the windows Mutex
- Option is simpler and more readable than Arc/Mutex nesting

**Alternatives considered:**
- `OnceLock`: Overkill for single-threaded initialization
- `Arc<Mutex<>>`: Adds unnecessary synchronization overhead when we already have locks

### 2. Clone device and queue for each Renderer

Pass cloned `wgpu::Device` and `wgpu::Queue` to each window's renderer. The wgpu crate implements `Clone` for these types, and they represent reference handles to underlying GPU resources, not the resources themselves.

**Rationale:**
- Cloning is cheap (reference counting, no deep copy)
- Each Renderer needs ownership for its lifetime
- wgpu docs confirm Clone is the correct pattern for sharing devices/queues
- Avoids complex lifetime management with references

**Alternatives considered:**
- Shared references (`&Device`): Would require complex lifetime management across Renderer/Gpu
- Arc: Unnecessary overhead for single-threaded use

### 3. Remove adapter from Renderer, pass reference when needed

Remove `adapter` from the `Gpu` struct since it's only needed for `get_capabilities()` calls during resize. Pass `&adapter` from the plugin when `resize()` is called rather than storing it.

**Rationale:**
- Adapter is only used during surface configuration (resize)
- Reduces state stored in each Renderer
- Plugin always has access to adapter, safe to pass reference

**Alternatives considered:**
- Store adapter in Renderer with Arc: Unnecessary overhead for single-threaded use
- Clone adapter: Adapter also implements Clone, but no need to own it per window

### 4. Keep GPU resources alive for application lifetime

Do not clean up instance/adapter/device/queue even when all windows are closed. Initialize on first window creation and keep forever.

**Rationale:**
- Simplest implementation
- Avoids complex state tracking (when to cleanup vs. when to reinitialize)
- wgpu resource cleanup happens when plugin is dropped at app shutdown
- Can optimize later if resource pressure becomes an issue

**Alternatives considered:**
- Cleanup when all windows close, reinitialize on next window: Adds complexity for minimal benefit
- Reference counting windows to trigger cleanup: Over-engineering for current use case

### 5. Refactor signatures to accept shared resources

Update `Renderer::new()` and `Gpu::new_async()` to accept `&instance`, `&adapter`, `device`, and `queue` as parameters rather than creating them internally.

**Rationale:**
- Makes resource ownership explicit
- Allows plugin to control resource lifecycle
- Clearer separation of concerns

**Signature changes:**
```rust
// Before
pub async fn new(window: impl Into<wgpu::SurfaceTarget<'static>>, width: u32, height: u32) -> Result<Self>

// After
pub async fn new(
    window: impl Into<wgpu::SurfaceTarget<'static>>,
    width: u32,
    height: u32,
    instance: &wgpu::Instance,
    adapter: &wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
) -> Result<Self>
```

## Risks / Trade-offs

[Risk] Device/Queue cloning may not work as expected if wgpu API changes
→ **Mitigation**: Verify Clone trait implementation in current wgpu version (docs confirm Clone is implemented)

[Risk] Surface lifecycle issues with shared instance across multiple windows
→ **Mitigation**: wgpu Surface is tied to specific window handle, instance is safe to share per wgpu design

[Risk] Thread safety issues if rendering ever becomes multi-threaded
→ **Mitigation**: Current design assumes single-threaded event loop. Future changes would need to wrap in Arc/Mutex if threading model changes

[Trade-off] Keeping GPU resources alive forever holds resources even when not needed
→ **Acceptance**: Simple implementation for now. Can add lazy cleanup later if resource pressure becomes a problem.

[Trade-off] Increased complexity in `create_iced_window` with initialization logic
→ **Acceptance**: One-time cost at first window, worth it for lifetime leak fix
