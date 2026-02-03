## Why

Memory leak occurs when creating and destroying windows repeatedly. Each window creates its own complete wgpu stack (instance, adapter, device, queue, surface), and these resources are never cleaned up properly when windows close. This causes observable memory growth in applications that frequently create/destroy windows.

## What Changes

- Move wgpu instance, adapter, device, queue to IcedPlugin as Option members
- Initialize shared GPU resources on first window creation
- Pass shared device and queue (cloned) to each window's renderer
- Remove adapter from Renderer - pass reference from plugin when needed
- Refactor Renderer::new to accept instance/adapter/device/queue as parameters
- Refactor Gpu::new_async to accept shared resources instead of creating its own
- Update resize method to accept adapter reference from plugin
- Keep adapter/device/queue alive for application lifetime (no cleanup on all windows closed)

## Capabilities

### New Capabilities

None (this is an implementation refactor only)

### Modified Capabilities

None (no spec-level behavior changes, internal refactor only)

## Impact

- **plugin.rs**: Add instance/adapter/device/queue Option members to IcedPlugin; modify create_iced_window to initialize on first window and pass resources to Renderer
- **renderer.rs**: Remove adapter from Gpu struct; update Renderer::new and Gpu::new_async signatures to accept instance/adapter/device/queue parameters; update resize to accept adapter reference
- **No user-facing API changes**: Plugin usage remains the same from application perspective
