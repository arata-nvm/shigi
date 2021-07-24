use crate::layout::Region;

pub fn calc_text_region(text: String) -> Region {
    let surface = cairo::ImageSurface::create(cairo::Format::A1, 256, 256).unwrap();
    let ctx = cairo::Context::new(&surface);

    let extents = ctx.text_extents(&text);
    Region::new(extents.width as f32, extents.height as f32)
}
