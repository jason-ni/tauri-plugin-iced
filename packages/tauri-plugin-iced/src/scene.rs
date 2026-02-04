use iced_winit::core::Color;

/// Scene trait for custom drawing to pixel buffer.
///
/// Unlike the wgpu-based implementation, this trait receives a tiny_skia PixmapMut
/// which allows direct CPU drawing operations. Scenes draw geometric shapes using
/// tiny_skia primitives (PathBuilder, Paint, etc.).
///
/// # Migration from wgpu:
/// Before: `fn draw(&self, render_pass: &mut wgpu::RenderPass)`
/// After:  `fn draw(&self, pixmap: &mut tiny_skia::PixmapMut, bg_color: Color)`
///
/// Example drawing:
/// ```rust
/// use tiny_skia::{PathBuilder, Paint, FillRule, Transform};
///
/// let path = PathBuilder::from_rect(Rect::from_xywh(...))?;
/// pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
/// ```
pub trait Scene: Send + Sync {
    fn draw(&self, pixmap: &mut tiny_skia::PixmapMut, bg_color: Color);
}
