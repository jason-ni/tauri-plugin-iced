## Why

The wgpu renderer has a memory leak that prevents reliable long-term usage. Migrating to tiny_skia eliminates this issue while significantly reducing complexity. The tiny_skia renderer is software-based and lightweight, making it ideal for our use case of simple geometric shapes without requiring GPU acceleration.

## What Changes

- **BREAKING**: Remove all wgpu dependencies from workspace and plugin
  - Remove `wgpu` crate
  - Remove `iced_wgpu` crate
  - Remove GPU initialization code (adapter, device, queue, instance)
- Add tiny_skia renderer dependencies
  - Add `iced_tiny_skia` with `geometry` feature
  - Add `softbuffer` for surface management
  - Add `tiny-skia` for drawing primitives
  - Add `bytemuck` for buffer casting
- Rewrite renderer.rs
  - Replace `GpuResource` with `SurfaceResource` (context + surface)
  - Replace GPU initialization with simple softbuffer setup
  - Remove command encoding and GPU pipeline management
- Rewrite plugin.rs
  - Remove async GPU initialization (~100 lines)
  - Replace `get_gpu_resources()` with synchronous surface creation
  - Remove GPU state fields from plugin struct
- Rewrite utils.rs render pipeline
  - Replace GPU texture operations with CPU pixel buffer operations
  - Implement softbuffer `buffer_mut()` → `PixmapMut` conversion
  - Replace GPU present with buffer present
- Rewrite scene.rs API
  - Change trait signature from `draw(&mut RenderPass)` to `draw(&mut PixmapMut)`
  - Remove wgpu-specific clear function
- Update event_conversion.rs
  - Change Viewport import from `iced_wgpu::graphics` to `iced_tiny_skia::graphics`

## Capabilities

### New Capabilities
- `tiny-skia-renderer`: Software-based rendering capability using tiny_skia and softbuffer for Iced UI with custom scene drawing support

### Modified Capabilities
- None (implementation change only, no spec-level behavior changes)

## Impact

**Code Changes:**
- Complete rewrite of `packages/tauri-plugin-iced/src/renderer.rs` (~120 lines)
- Complete rewrite of `packages/tauri-plugin-iced/src/plugin.rs` (~450 lines)
- Complete rewrite of `packages/tauri-plugin-iced/src/utils.rs` (~236 lines)
- Complete rewrite of `packages/tauri-plugin-iced/src/scene.rs` (~39 lines)
- Update `packages/tauri-plugin-iced/src/event_conversion.rs` (minor import changes)
- Update `packages/tauri-plugin-iced/Cargo.toml` (dependency changes)
- Update `Cargo.toml` workspace (dependency changes)

**Dependency Changes:**
- Remove: `wgpu`, `iced_wgpu`
- Add: `iced_tiny_skia`, `softbuffer`, `tiny-skia`, `bytemuck`

**API Breaking Changes:**
- `Scene` trait: `draw(&mut wgpu::RenderPass)` → `draw(&mut tiny_skia::PixmapMut)`
- Existing scene implementations will need to be updated to CPU rendering

**Performance Changes:**
- Startup: Faster (no async GPU initialization)
- Rendering: Slower (CPU-bound vs GPU-accelerated)
- Memory: Fixed (eliminates wgpu memory leak)
- Binary size: Smaller (no wgpu dependencies)

**Migration Complexity:**
- High - complete rewrite of rendering pipeline
- Requires updating all custom scene implementations
- No data migration needed (pure implementation change)
