use crate::layout::Region;

pub fn calc_text_region(text: String, font_size: f32) -> Region {
    let surface = cairo::ImageSurface::create(cairo::Format::A1, 256, 256).unwrap();
    let ctx = cairo::Context::new(&surface);

    ctx.set_font_size(font_size as f64);
    let extents = ctx.text_extents(&text);
    Region::new(extents.x_advance as f32, extents.height as f32)
}
