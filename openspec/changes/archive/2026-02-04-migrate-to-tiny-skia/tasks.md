## 1. Setup and Dependencies

- [x] 1.1 Update workspace Cargo.toml to remove wgpu dependencies
- [x] 1.2 Add iced_tiny_skia, softbuffer, tiny-skia, bytemuck to workspace dependencies
- [x] 1.3 Update plugin Cargo.toml to remove wgpu, iced_wgpu
- [x] 1.4 Add iced_tiny_skia with geometry feature to plugin dependencies
- [x] 1.5 Add softbuffer, tiny-skia, bytemuck to plugin dependencies
- [ ] 1.6 Configure softbuffer platform features (x11/wayland) for Linux if needed

## 2. Renderer Implementation

- [x] 2.1 Create SurfaceResource struct in renderer.rs with context and surface fields
- [x] 2.2 Implement SurfaceResource::new() to create softbuffer Context and Surface
- [x] 2.3 Implement SurfaceResource::resize() using softbuffer surface.resize()
- [x] 2.4 Implement SurfaceResource::get_buffer_mut() to return softbuffer buffer
- [x] 2.5 Update Renderer struct to use iced_tiny_skia::Renderer and SurfaceResource
- [x] 2.6 Rewrite Renderer::new() to create tiny_skia renderer (remove GPU parameters)
- [x] 2.7 Update Renderer::iced_renderer() to return &mut iced_tiny_skia::Renderer
- [x] 2.8 Update Renderer::surface_resource() to return &SurfaceResource
- [x] 2.9 Remove all wgpu-related code from renderer.rs (GpuResource, Engine, etc.)

## 3. Plugin Simplification

- [x] 3.1 Remove GPU state fields from IcedPlugin struct (instance, adapter, device, queue)
- [x] 3.2 Implement get_surface_resources() to create softbuffer context and surface
- [x] 3.3 Replace async GPU initialization block with synchronous surface creation
- [x] 3.4 Update RedrawRequested handler to use get_surface_resources()
- [x] 3.5 Simplify renderer initialization (remove adapter parameter)
- [x] 3.6 Update SurfaceResource struct to use tauri::Window types
- [x] 3.7 Remove get_gpu_resources() function (~100 lines removed)
- [x] 3.8 Verify import statements updated (remove wgpu imports)

## 4. Render Pipeline Implementation

- [x] 4.1 Update IcedWindow struct to use new Renderer type
- [x] 4.2 Rewrite render() method resize logic to use SurfaceResource::resize()
- [x] 4.3 Implement pixel buffer retrieval using surface.buffer_mut()
- [x] 4.4 Create PixmapMut from buffer bytes using bytemuck::cast_slice_mut()
- [x] 4.5 Remove wgpu command encoder creation
- [x] 4.6 Remove wgpu render pass creation
- [x] 4.7 Update interface.draw() call to use tiny_skia renderer
- [x] 4.8 Add renderer.draw() compositing call with PixmapMut and parameters
- [x] 4.9 Update scene drawing to use PixmapMut instead of RenderPass
- [x] 4.10 Replace wgpu present with buffer.present()
- [x] 4.11 Update error handling for softbuffer SurfaceError
- [x] 4.12 Remove FrameAndView struct (no longer needed)

## 5. Scene API Migration

- [x] 5.1 Update Scene trait signature to use PixmapMut and bg_color
- [x] 5.2 Remove clear() function from scene.rs (background handled by tiny_skia)
- [x] 5.3 Create example scene implementation using tiny_skia drawing primitives
- [x] 5.4 Update scene.draw() to use tiny_skia::PathBuilder, Paint, PixmapMut.fill_path()
- [x] 5.5 Document scene API migration pattern in comments

## 6. Event Conversion Updates

- [x] 6.1 Update Viewport import from iced_wgpu::graphics to iced_tiny_skia::graphics
- [x] 6.2 Verify create_viewport() function works with new Viewport type
- [x] 6.3 Remove any wgpu-specific event handling logic
- [x] 6.4 Test event conversion with new renderer type

## 7. Public API Updates

- [x] 7.1 Update lib.rs exports if needed (remove wgpu exports)
- [x] 7.2 Verify IcedControls trait still compatible
- [x] 7.3 Update AppHandleExt::create_iced_window() if renderer init changed
- [x] 7.4 Verify IcedWindow struct still implements required traits

## 8. Documentation and Comments

- [ ] 8.1 Add documentation for SurfaceResource struct
- [ ] 8.2 Add documentation for renderer initialization flow
- [ ] 8.3 Document softbuffer usage pattern in render pipeline
- [ ] 8.4 Add comments explaining CPU rendering vs GPU rendering differences
- [ ] 8.5 Document scene API migration in scene.rs

## 9. Build and Compile

- [ ] 9.1 Run cargo build on plugin package
- [ ] 9.2 Fix any compilation errors related to type changes
- [ ] 9.3 Fix any compilation errors related to missing imports
- [ ] 9.4 Verify all warnings are addressed
- [ ] 9.5 Run cargo build on entire workspace

## 10. Testing - macOS

- [ ] 10.1 Run test application on macOS
- [ ] 10.2 Verify window renders correctly
- [ ] 10.3 Verify resize handling works
- [ ] 10.4 Verify mouse cursor updates work
- [ ] 10.5 Verify custom scene draws correctly
- [ ] 10.6 Check for memory leaks (monitor memory usage)

## 11. Testing - Linux

- [ ] 11.1 Run test application on Linux
- [ ] 11.2 Verify window renders correctly
- [ ] 11.3 Verify resize handling works
- [ ] 11.4 Verify mouse cursor updates work
- [ ] 11.5 Verify custom scene draws correctly
- [ ] 11.6 Check softbuffer x11/wayland features work correctly

## 12. Testing - Windows

- [ ] 12.1 Run test application on Windows
- [ ] 12.2 Verify window renders correctly
- [ ] 12.3 Verify resize handling works
- [ ] 12.4 Verify mouse cursor updates work
- [ ] 12.5 Verify custom scene draws correctly
- [ ] 12.6 Check for any Windows-specific softbuffer issues

## 13. Performance Validation

- [ ] 13.1 Measure startup time (expect faster than wgpu)
- [ ] 13.2 Measure rendering performance for simple UI
- [ ] 13.3 Measure rendering performance with custom scene
- [ ] 13.4 Check binary size reduction
- [ ] 13.5 Verify no memory leaks (compare to wgpu baseline)

## 14. Code Cleanup

- [ ] 14.1 Remove any unused imports
- [ ] 14.2 Remove any commented-out wgpu code
- [ ] 14.3 Run cargo clippy and fix warnings
- [ ] 14.4 Run cargo fmt to ensure consistent formatting
- [ ] 14.5 Verify code follows project conventions

## 15. Final Verification

- [ ] 15.1 Test all example scenes work correctly
- [ ] 15.2 Verify no wgpu dependencies remain in Cargo.lock
- [ ] 15.3 Confirm memory leak is resolved
- [ ] 15.4 Document any known limitations or edge cases
- [ ] 15.5 Prepare migration guide for custom scene implementations
