use crate::display::{DisplayCommand, DisplayList};
use crate::layout::Rect;
use cairo::{Context, PdfSurface};

pub struct PdfRenderer {
    ctx: Context,
    pub width: f32,
    pub height: f32,
}

pub fn render(display_list: &DisplayList, bounds: Rect, file_name: String) {
    let mut renderer = PdfRenderer::new(bounds.width, bounds.height, file_name);
    for item in display_list {
        renderer.render_item(&item);
    }
}

impl PdfRenderer {
    pub fn new(width: f32, height: f32, file_name: String) -> PdfRenderer {
        let surface = PdfSurface::new(width as f64, height as f64, file_name).unwrap();
        let ctx = Context::new(&surface);
        PdfRenderer {
            ctx: ctx,
            width: width,
            height: height,
        }
    }

    fn render_item(&mut self, item: &DisplayCommand) {}
}
