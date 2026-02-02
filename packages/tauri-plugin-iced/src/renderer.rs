use anyhow::Error;
use iced::Renderer as IcedRenderer;
use iced_wgpu::{wgpu, Engine, Renderer as WgpuRenderer};
use iced_winit::core::{Font, Pixels};

pub struct Renderer {
    pub renderer: IcedRenderer,
    pub gpu: Gpu,
}

pub struct FrameAndView {
    pub surface_texture: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
}

pub struct Gpu {
    _instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    surface: wgpu::Surface<'static>,
    queue: wgpu::Queue,
    device: wgpu::Device,
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

        let renderer = IcedRenderer::Primary(WgpuRenderer::new(engine, Font::default(), Pixels::from(16)));

        Ok(Self { gpu, renderer })
    }

    pub fn iced_renderer(&mut self) -> &mut IcedRenderer {
        &mut self.renderer
    }

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
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

        let adapter =
            wgpu::util::initialize_adapter_from_env_or_default(&instance, Some(&surface)).await?;

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
            .find(|f| !matches!(f, wgpu::TextureFormat::Bgra8UnormSrgb | wgpu::TextureFormat::Rgba8UnormSrgb))
            .unwrap_or_else(|| surface_capabilities.formats[0]);

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
            _instance: instance,
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

    pub fn get_frame(&self) -> Result<FrameAndView, wgpu::SurfaceError> {
        let surface_texture = self.surface.get_current_texture()?;

        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Ok(FrameAndView {
            surface_texture,
            view,
        })
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn format(&self) -> wgpu::TextureFormat {
        self.surface_format
    }
}

