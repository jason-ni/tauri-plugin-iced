## Context

**Current State:**
The plugin uses wgpu for GPU-accelerated rendering with a complex initialization pipeline:
- Async GPU initialization requiring ~100 lines of code
- GPU state management (instance, adapter, device, queue, surface)
- Command encoding and render pass management
- Texture/frame buffer management

**Problem:**
The wgpu renderer has a memory leak that prevents reliable long-term usage. This is a critical issue requiring complete abandonment of the wgpu approach.

**Constraints:**
- Must remove ALL wgpu dependencies (no hybrid approach)
- Must maintain compatibility with Tauri event loop integration
- Custom scene rendering support required for geometric shapes
- Lightweight rendering sufficient (simple UI only)

**Reference Implementation:**
The `integration_skia` example demonstrates tiny_skia + softbuffer integration with winit, providing a clear migration pattern.

## Goals / Non-Goals

**Goals:**
1. Eliminate wgpu memory leak by complete removal
2. Simplify renderer architecture (remove ~200 lines of GPU initialization code)
3. Reduce startup time (no async GPU init)
4. Reduce binary size (no wgpu dependencies)
5. Maintain Iced UI rendering capabilities
6. Maintain custom scene drawing support
7. Maintain Tauri window integration

**Non-Goals:**
1. Hybrid wgpu + tiny_skia approach (must fully abandon wgpu)
2. GPU acceleration for rendering (accept CPU-bound performance)
3. Maintaining wgpu API compatibility (breaking changes acceptable)
4. Performance optimization for complex scenes (lightweight use case)

## Decisions

### 1. Renderer Architecture: Direct tiny_skia Integration

**Decision:** Use `iced_tiny_skia::Renderer` directly instead of wrapper pattern.

**Rationale:**
- Integration_skia example uses this pattern successfully
- tiny_skia renderer is self-contained (no Engine/Device/Queue needed)
- Simpler than maintaining custom wrapper

**Alternatives Considered:**
- Custom wrapper maintaining wgpu-like API: More code to maintain, no benefit
- wgpu-like abstraction layer: Over-engineering for single renderer

### 2. Surface Management: Softbuffer Direct Integration

**Decision:** Use `softbuffer::Surface<Window, Window>` for direct Tauri window integration.

**Rationale:**
- Integration_skia uses `Arc<winit::window::Window>` successfully
- Tauri `Window` implements required Display trait
- Simplest approach with minimal abstraction layers

**Alternatives Considered:**
- winit window wrapper: Unnecessary complexity, Tauri window already compatible
- Custom surface abstraction: Over-engineering, softbuffer handles platforms

### 3. Render Pipeline: CPU Buffer → PixmapMut → Present

**Decision:** Follow integration_skia pattern exactly:
1. `surface.buffer_mut()` → mutable byte slice
2. `PixmapMut::from_bytes()` → tiny_skia drawing target
3. `interface.draw()` → populates renderer layers
4. `renderer.draw(pixmap, mask, viewport, damage, bg)` → CPU rasterization
5. `scene.draw(pixmap)` → custom scene on top
6. `buffer.present()` → display

**Rationale:**
- Proven pattern from reference implementation
- Explicit damage tracking for optimization
- Simple, synchronous flow

**Alternatives Considered:**
- GPU command queue approach: Not applicable to software renderer
- Double buffering with age tracking: Complex, softbuffer handles this

### 4. Scene API: PixmapMut Parameter

**Decision:** Break Scene trait signature:
```rust
// Before
fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);

// After
fn draw(&self, pixmap: &mut tiny_skia::PixmapMut, bg_color: Color);
```

**Rationale:**
- Direct drawing to pixel buffer required
- No GPU pipeline stages
- Matches tiny_skia's drawing model

**Breaking Change Impact:**
All custom scene implementations must be rewritten to use tiny_skia drawing primitives.

### 5. Plugin State: Eliminate GPU Fields

**Decision:** Remove all GPU state from `IcedPlugin`:
```rust
// Remove these fields:
instance: RefCell<Option<wgpu::Instance>>,
adapter: RefCell<Option<wgpu::Adapter>>,
device: RefCell<Option<wgpu::Device>>,
queue: RefCell<Option<wgpu::Queue>>,
```

**Rationale:**
- No GPU resources needed
- Reduces complexity significantly
- Eliminates async initialization

### 6. Error Handling: Simplified SurfaceError

**Decision:** Replace wgpu SurfaceError handling with softbuffer pattern:
```rust
match surface.buffer_mut() {
    Ok(buffer) => { render... }
    Err(_) => {
        window.request_redraw();
        return None;
    }
}
```

**Rationale:**
- Softbuffer errors are transient (no OutOfMemory fatal errors)
- Simple retry pattern sufficient
- Less error type complexity

**Alternatives Considered:**
- Detailed error categorization: Unnecessary, softbuffer has few error types
- Panic on error: Too aggressive, errors can be transient

### 7. Initialization Flow: Synchronous on RedrawRequested

**Decision:** Initialize renderer synchronously in `RedrawRequested` handler:
```rust
if iced_window.renderer.is_none() {
    let context = softbuffer::Context::new(window)?;
    let surface = softbuffer::Surface::new(&context, window)?;
    let renderer = iced_tiny_skia::Renderer::new(...);
    iced_window.renderer = Some(Renderer::new(surface_resource));
}
```

**Rationale:**
- No async operations needed
- Same pattern as integration_skia
- Fail-fast if window not ready

**Alternatives Considered:**
- Async initialization in `AppHandleExt`: Unnecessary complexity, synchronous is fine
- Pre-initialization before first draw: Not needed, lazy init is simpler

## Risks / Trade-offs

### Risk 1: Performance Degradation

[Performance degradation due to CPU-bound rendering] → Accept tradeoff given lightweight use case and memory leak severity. Software rendering is sufficient for simple geometric shapes.

### Risk 2: Breaking Scene API

[Existing scene implementations must be rewritten] → Document migration pattern clearly in scene.rs comments. Provide examples from integration_skia/src/scene.rs.

### Risk 3: Tauri Window Compatibility

[Softbuffer may have issues with Tauri window handles] → Integration_skia uses winit successfully. Tauri uses tao (fork of winit), should be compatible. Test thoroughly on all platforms.

### Risk 4: Buffer Resize Edge Cases

[Resize during rendering may cause buffer issues] → Softbuffer handles resize gracefully. Integration_skia pattern of resize → request_redraw works well.

### Trade-off: Simplification vs Flexibility

[Removing GPU pipeline limits future graphics features] → Accept tradeoff. Current requirements are simple. If needed in future, can add back wgpu as separate renderer (not in scope).

### Trade-off: Sync vs Async

[Synchronous rendering blocks event loop] → Software rendering is fast enough for simple UI. No frame-rate concerns for lightweight use case.

## Migration Plan

### Phase 1: Dependency Updates
1. Update workspace Cargo.toml:
   - Remove wgpu, iced_wgpu from workspace dependencies
   - Add iced_tiny_skia, softbuffer, tiny-skia, bytemuck
2. Update plugin Cargo.toml:
   - Remove wgpu, iced_wgpu
   - Add tiny_skia renderer dependencies
   - Configure iced_tiny_skia features (geometry, maybe image)

### Phase 2: Renderer Rewrite
1. Rewrite renderer.rs:
   - Replace GpuResource with SurfaceResource
   - Update Renderer::new() to create tiny_skia renderer
   - Remove all wgpu-specific code
2. Update surface_resource API:
   - resize() → softbuffer surface.resize()
   - get_buffer() → surface.buffer_mut()

### Phase 3: Plugin Simplification
1. Rewrite plugin.rs GPU initialization section:
   - Remove get_gpu_resources() (~100 lines)
   - Replace with get_surface_resources() (~10 lines)
   - Remove async block
   - Remove GPU state fields from IcedPlugin
2. Update renderer initialization in RedrawRequested:
   - Simplified synchronous initialization
   - Remove adapter parameter

### Phase 4: Render Pipeline Rewrite
1. Rewrite utils.rs render():
   - Replace command encoding with buffer access
   - Implement PixmapMut creation from buffer
   - Replace GPU present with buffer present
   - Add renderer.draw() compositing call
2. Update scene drawing order:
   - interface.draw() → renderer.draw(pixmap) → scene.draw(pixmap)

### Phase 5: Scene API Migration
1. Rewrite scene.rs:
   - Update Scene trait signature
   - Remove clear() function (handled by tiny_skia)
   - Update sample scene implementation

### Phase 6: Cleanup and Testing
1. Remove unused imports
2. Update event_conversion.rs imports
3. Test on all platforms (macOS, Linux, Windows)
4. Verify memory leak resolution
5. Performance benchmarking (baseline)

### Rollback Strategy
- Keep git history for easy revert
- No database/state changes (pure implementation)
- Can rollback by reverting dependency changes and restoring wgpu code

## Open Questions

1. **Q: Should we enable iced_tiny_skia features beyond `geometry`?**
   - A: Depends on if we need images or SVG support. Start with geometry only, add if needed.

2. **Q: Will softbuffer work with Tauri's tao window on all platforms?**
   - A: tao is a winit fork. Integration_skia uses winit successfully. Need to verify.

3. **Q: Should we expose damage rectangle optimization or redraw everything?**
   - A: Start with full redraw (simpler). Optimize with damage tracking if performance issue.

4. **Q: How to handle window transparency with softbuffer?**
   - A: Integration_skia doesn't show transparency handling. May need investigation or accept opaque rendering.

5. **Q: Should we keep scene.draw() calling order or match integration_skia exactly?**
   - A: Integration_skia draws scene AFTER Iced UI. Our current wgpu draws scene BEFORE. Follow integration_skia pattern (scene on top).
