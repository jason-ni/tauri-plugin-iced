## 1. Plugin Structure Changes

- [x] 1.1 Add `instance: Option<wgpu::Instance>` field to `IcedPlugin` struct
- [x] 1.2 Add `adapter: Option<wgpu::Adapter>` field to `IcedPlugin` struct
- [x] 1.3 Add `device: Option<wgpu::Device>` field to `IcedPlugin` struct
- [x] 1.4 Add `queue: Option<wgpu::Queue>` field to `IcedPlugin` struct

## 2. Plugin Initialization Logic

- [x] 2.1 In `create_iced_window`, add check: if `self.device.is_none()` then initialize GPU resources
- [x] 2.2 Create `wgpu::Instance` with backend configuration
- [x] 2.3 Create `wgpu::Adapter` using `initialize_adapter_from_env_or_default` with instance
- [x] 2.4 Request `wgpu::Device` and `wgpu::Queue` from adapter
- [x] 2.5 Store instance, adapter, device, queue in plugin Option fields
- [x] 2.6 Wrap GPU initialization in `tauri::async_runtime::block_on` async block

## 3. Renderer/Gpu Refactoring

- [ ] 3.1 Remove `adapter: wgpu::Adapter` field from `Gpu` struct in renderer.rs
- [ ] 3.2 Change `_instance: wgpu::Instance` to `Option<wgpu::Instance>` in `Gpu` struct
- [ ] 3.3 Update `Renderer::new()` signature to accept `instance: &wgpu::Instance`, `adapter: &wgpu::Adapter`, `device: wgpu::Device`, `queue: wgpu::Queue` parameters
- [ ] 3.4 Update `Gpu::new_async()` signature to accept `instance: &wgpu::Instance`, `adapter: &wgpu::Adapter`, `device: wgpu::Device`, `queue: wgpu::Queue` parameters
- [ ] 3.5 Remove instance creation code from `Gpu::new_async()` (now received as parameter)
- [ ] 3.6 Remove adapter creation code from `Gpu::new_async()` (now received as parameter)
- [ ] 3.7 Remove device/queue request code from `Gpu::new_async()` (now received as parameter)
- [ ] 3.8 Keep surface creation using `instance.create_surface(window)?` with passed instance
- [ ] 3.9 Keep surface configuration using `adapter.get_capabilities(&surface)` with passed adapter
- [ ] 3.10 Update `Gpu::resize()` signature to accept `adapter: &wgpu::Adapter` parameter
- [ ] 3.11 Update `resize()` to use passed adapter for `surface.get_capabilities(adapter)`

## 4. Plugin Integration

- [x] 4.1 Update `Renderer::new()` call in `create_iced_window` to pass instance/adapter/device/queue from plugin
- [x] 4.2 Update resize handling in plugin `on_event` to pass adapter reference to `resize()` call
- [x] 4.3 Verify all calls to Renderer methods that need adapter now receive it from plugin

## 5. Testing and Verification

- [ ] 5.1 Test creating first window - verify GPU resources initialized correctly
- [ ] 5.2 Test creating multiple windows - verify shared resources work
- [ ] 5.3 Test destroying windows - verify no memory growth with repeated create/destroy cycles
- [ ] 5.4 Test window resize - verify adapter reference passed correctly to resize()
- [ ] 5.5 Run project build to verify compilation succeeds
- [ ] 5.6 Run project tests to verify no regressions
