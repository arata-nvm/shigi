extern crate shigi;

mod clap_app;

use std::fs;
use shigi::display::build_display_list;
use shigi::layout::layout_tree;
use shigi::pdf::render;
use shigi::style::style_tree;
use shigi::{css, html};

fn main() {
    let matches = clap_app::build_app()
        .get_matches();

    let html_file = matches.value_of("html-file").unwrap();
    let css_file = matches.value_of("css-file").unwrap();
    let output = matches.value_of("output").unwrap();

    let html_source = fs::read_to_string(html_file).unwrap();
    let css_source = fs::read_to_string(css_file).unwrap();

    render_to_pdf(html_source, css_source, output.to_string());
}

fn render_to_pdf(html_source: String, css_source: String, output: String) {
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

    let nodes = html::parse(html_source);
    let stylesheet = css::parse(css_source);
    let style_tree = style_tree(&nodes, &stylesheet);
    let layout_root = layout_tree(&style_tree, initial_containing_block);
    let display_list = build_display_list(&layout_root);
    render(
        &display_list,
        initial_containing_block.content,
        output,
    );
}

