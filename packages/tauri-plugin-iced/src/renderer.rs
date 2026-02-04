use anyhow::Error;
use iced_tiny_skia::Renderer as TinySkiaRenderer;
use iced_winit::core::{Font, Pixels};
use std::num::NonZeroU32;
use std::sync::Arc;
use tauri::Window;

pub struct IcedRenderer {
    pub renderer: TinySkiaRenderer,
    pub surface_resource: SurfaceResource,
}

/// Surface resource for managing softbuffer context and surface.
///
/// Wraps softbuffer's Context and Surface for window pixel buffer management.
pub struct SurfaceResource {
    context: Arc<softbuffer::Context<Window>>,
    window: Arc<Window>,
    surface: Option<softbuffer::Surface<Window, Arc<Window>>>,
}

impl IcedRenderer {
    /// Create a new renderer with the given surface resource.
    ///
    /// Initializes a tiny_skia software renderer and attaches it to the softbuffer surface.
    /// The renderer performs CPU-based rendering to the window pixel buffer.
    pub fn new(surface_resource: SurfaceResource) -> Result<Self, Error> {
        let renderer = TinySkiaRenderer::new(Font::default(), Pixels::from(16));

        Ok(Self {
            renderer,
            surface_resource,
        })
    }

    pub fn tiny_skia_renderer(&mut self) -> &mut TinySkiaRenderer {
        &mut self.renderer
    }

    pub fn surface_resource(&mut self) -> &mut SurfaceResource {
        &mut self.surface_resource
    }
}

impl SurfaceResource {
    pub fn new(context: softbuffer::Context<Window>, window: Arc<Window>) -> Self {
        Self {
            context: Arc::new(context),
            window,
            surface: None,
        }
    }

    fn ensure_surface(&mut self) {
        if self.surface.is_none() {
            let surface = softbuffer::Surface::new(&self.context, self.window.clone());
            if surface.is_ok() {
                self.surface = Some(surface.unwrap());
            }
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.ensure_surface();
        if let Some(surface) = &mut self.surface {
            let width_nz = NonZeroU32::new(width).unwrap_or(NonZeroU32::new(1).unwrap());
            let height_nz = NonZeroU32::new(height).unwrap_or(NonZeroU32::new(1).unwrap());
            let _ = surface.resize(width_nz, height_nz);
        }
    }

    pub fn get_buffer_mut(&mut self) -> Result<softbuffer::Buffer<'_, Window, Arc<Window>>, Error> {
        self.ensure_surface();
        let surface = self
            .surface
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("Surface not available"))?;
        surface
            .buffer_mut()
            .map_err(|e| anyhow::anyhow!("Failed to get buffer: {:?}", e))
    }
}
