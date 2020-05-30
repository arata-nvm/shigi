extern crate shigi;

use shigi::display::build_display_list;
use shigi::layout::layout_tree;
use shigi::pdf::render;
use shigi::style::style_tree;
use shigi::{css, html};

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

    let initial_containing_block = shigi::layout::Dimensions {
        content: shigi::layout::Rect {
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 600.0,
        },
        padding: Default::default(),
        border: Default::default(),
        margin: Default::default(),
    };

    let nodes = html::parse(html_source);
    let stylesheet = css::parse(css_source);
    let style_tree = style_tree(&nodes, &stylesheet);
    let layout_root = layout_tree(&style_tree, initial_containing_block);
    let display_list = build_display_list(&layout_root);
    render(
        &display_list,
        initial_containing_block.content,
        "output.pdf".to_string(),
    );
}
