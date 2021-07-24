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

    fn render_item(&mut self, item: &DisplayCommand) {
        match item {
            DisplayCommand::SolidColor(ref color, ref rect) => {
                self.ctx.set_source_rgba(
                    color.r as f64 / 255.0,
                    color.g as f64 / 255.0,
                    color.b as f64 / 255.0,
                    color.a as f64 / 255.0,
                );
                self.ctx.rectangle(
                    rect.x as f64,
                    rect.y as f64,
                    rect.width as f64,
                    rect.height as f64,
                );
                self.ctx.fill();
            }
            DisplayCommand::Text(ref text, ref pos) => {
                let extents = self.ctx.text_extents(text);
                let y = pos.y as f64 - extents.y_bearing;
                self.ctx.move_to(pos.x as f64, y);
                self.ctx.show_text(text);
            }
        }
    }
}
