extern crate shigi;

mod clap_app;

use shigi::display::build_display_list;
use shigi::layout::layout_tree;
use shigi::pdf::render;
use shigi::style::style_tree;
use shigi::{css, dom, html};
use std::fs;

fn main() {
    let matches = clap_app::build_app().get_matches();

    let html_path = matches.value_of("html-file").unwrap();
    let output = matches.value_of("output").unwrap();
    render_to_pdf(html_path, output);
}

fn render_to_pdf<S: Into<String>>(html_path: S, output_path: S) {
    let html_path = html_path.into();
    let output_path = output_path.into();

    // TODO: Don't use magic numbers
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

    let html_source = fs::read_to_string(html_path).unwrap();
    let node = html::parse(html_source);

    let stylesheet = match find_stylesheet(&node) {
        Some(css_path) => {
            let css_source = fs::read_to_string(css_path).unwrap();
            css::parse(css_source)
        }
        None => css::Stylesheet::default(),
    };

    let style_tree = style_tree(&node, &stylesheet);
    let layout_root = layout_tree(&style_tree, initial_containing_block);
    let display_list = build_display_list(&layout_root);
    render(&display_list, initial_containing_block.content, output_path);
}

fn find_stylesheet(node: &dom::Node) -> Option<&String> {
    if let dom::NodeType::Element(dat) = &node.typ {
        if dat.tag_name == "link" && dat.attrs["rel"] == "stylesheet" {
            return Some(dat.attrs.get("href").unwrap());
        }
    }

    for child in &node.children {
        if let Some(path) = find_stylesheet(child) {
            return Some(path);
        }
    }

    return None;
}
