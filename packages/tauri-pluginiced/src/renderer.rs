use anyhow::Error;
use iced_wgpu::{Engine, Renderer as IcedRenderer, wgpu};
use iced_winit::core::{Font, Pixels};

pub struct Renderer {
    pub gpu: Gpu,
    pub renderer: IcedRenderer,
}

pub struct Gpu {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
}

impl Renderer {
    pub async fn new(
        window: impl Into<wgpu::SurfaceTarget<'static>>,
        width: u32,
        height: u32,
    ) -> Result<Self, Error> {
        let gpu = Gpu::new_async(window, width, height).await?;

        let engine = Engine::new(
            &gpu.adapter,
            gpu.device.clone(),
            gpu.queue.clone(),
            gpu.surface_format,
            None,
            iced_wgpu::graphics::Shell::headless(),
        );

        let renderer = IcedRenderer::new(engine, Font::default(), Pixels::from(16));

        Ok(Self { gpu, renderer })
    }
}

impl Gpu {
    pub async fn new_async(
        window: impl Into<wgpu::SurfaceTarget<'static>>,
        width: u32,
        height: u32,
    ) -> Result<Self, Error> {
        let backend = wgpu::Backends::from_env().unwrap_or_default();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: backend,
            ..Default::default()
        });
        let surface = instance.create_surface(window)?;

        let adapter = wgpu::util::initialize_adapter_from_env_or_default(&instance, Some(&surface))
            .await?;

        let adapter_features = adapter.features();

        let surface_capabilities = surface.get_capabilities(&adapter);

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: adapter_features & wgpu::Features::default(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                trace: wgpu::Trace::Off,
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
            })
            .await?;

        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .or_else(|| surface_capabilities.formats.first().copied())
            .ok_or_else(|| anyhow::anyhow!("No supported texture format"))?;

        let alpha_mode = surface_capabilities
            .alpha_modes
            .iter()
            .copied()
            .find(|m| *m != wgpu::CompositeAlphaMode::Opaque)
            .unwrap_or(surface_capabilities.alpha_modes[0]);

        surface.configure(
            &device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width,
                height,
                present_mode: wgpu::PresentMode::AutoVsync,
                alpha_mode,
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            },
        );

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
            surface,
            surface_format,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let surface_capabilities = self.surface.get_capabilities(&self.adapter);

        self.surface.configure(
            &self.device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: self.surface_format,
                width,
                height,
                present_mode: surface_capabilities
                    .present_modes
                    .first()
                    .copied()
                    .unwrap_or(wgpu::PresentMode::AutoVsync),
                alpha_mode: surface_capabilities
                    .alpha_modes
                    .first()
                    .copied()
                    .unwrap_or(wgpu::CompositeAlphaMode::Auto),
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            },
        );
    }
}
