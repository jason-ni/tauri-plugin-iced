use anyhow::Error;
use iced::Renderer as IcedRenderer;
use iced_wgpu::{wgpu, Engine, Renderer as WgpuRenderer};
use iced_winit::core::{Font, Pixels};

pub struct Renderer {
    pub renderer: IcedRenderer,
    pub gpu_resource: GpuResource,
}

pub struct FrameAndView {
    pub surface_texture: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
}

pub struct GpuResource {
    surface_format: wgpu::TextureFormat,
    surface_capabilities: wgpu::SurfaceCapabilities,
    surface: wgpu::Surface<'static>,
    queue: wgpu::Queue,
    device: wgpu::Device,
}

impl Renderer {
    pub fn new(
        adapter: &wgpu::Adapter,
        gpu_resource: GpuResource,
    ) -> Result<Self, Error> {

        let engine = Engine::new(
            adapter,
            gpu_resource.device.clone(),
            gpu_resource.queue.clone(),
            gpu_resource.surface_format,
            None,
            iced_wgpu::graphics::Shell::headless(),
        );

        let renderer = IcedRenderer::Primary(WgpuRenderer::new(engine, Font::default(), Pixels::from(16)));

        Ok(Self { renderer, gpu_resource })
    }

    pub fn iced_renderer(&mut self) -> &mut IcedRenderer {
        &mut self.renderer
    }

    pub fn gpu_resource(&self) -> &GpuResource {
        &self.gpu_resource
    }
}

impl GpuResource {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        surface: wgpu::Surface<'static>,
        surface_format: wgpu::TextureFormat,
        surface_capabilities: wgpu::SurfaceCapabilities,
    ) -> Self {

        Self {
            surface,
            surface_format,
            surface_capabilities,
            queue,
            device,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {

        self.surface.configure(
            &self.device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: self.surface_format,
                width,
                height,
                present_mode: self.surface_capabilities
                    .present_modes
                    .first()
                    .copied()
                    .unwrap_or(wgpu::PresentMode::AutoVsync),
                alpha_mode: self.surface_capabilities
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

