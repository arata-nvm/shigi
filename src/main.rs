use crate::style::style_tree;
use crate::layout::layout_tree;
use crate::painting::paint;
use std::fs::File;
use std::path::Path;
extern crate image;

pub mod css;
pub mod dom;
pub mod html;
pub mod layout;
pub mod painting;
pub mod style;

fn main() {
    let html_source = r#"
<div class="a">
  <div class="b">
    <div class="c">
      <div class="d">
        <div class="e">
          <div class="f">
            <div class="g">
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>"#
        .to_string();

    let css_source = r#"
* { display: block; padding: 12px; }
.a { background: #ff0000; }
.b { background: #ffa500; }
.c { background: #ffff00; }
.d { background: #008000; }
.e { background: #0000ff; }
.f { background: #4b0082; }
.g { background: #800080; }"#
        .to_string();

    let initial_containing_block = layout::Dimensions {
        content: layout::Rect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 },
        padding: Default::default(),
        border: Default::default(),
        margin: Default::default(),
    };

    let nodes = html::Parser::parse(html_source);
    let stylesheet = css::Parser::parse(css_source);
    let style_tree = style_tree(&nodes, &stylesheet);
    let layout_root = layout_tree(&style_tree, initial_containing_block);
    let canvas = paint(&layout_root, initial_containing_block.content);

    let filename = "output.png";
    let mut file = File::create(&Path::new(filename)).unwrap();

    // Save an image:
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    let buffer: Vec<image::Rgba<u8>> = unsafe { std::mem::transmute(canvas.pixels) };
    let img = image::ImageBuffer::from_fn(w, h, Box::new(|x: u32, y: u32| buffer[(y * w + x) as usize]));

    let result = image::ImageRgba8(img).save(&mut file, image::PNG);

}
