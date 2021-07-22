extern crate shigi;

mod clap_app;

use shigi::layout::{Dimensions, Rect};
use shigi::{css, display, html, layout, pdf, style};
use std::fs;

fn main() {
    let matches = clap_app::build_app().get_matches();
    let html_path = matches.value_of("html-file").unwrap();
    let output = matches.value_of("output").unwrap();

    let bound = Dimensions::new(Rect::new(0.0, 0.0, 800.0, 600.0));
    render_to_pdf(html_path, output, bound);
}

fn render_to_pdf<S: Into<String>>(html_path: S, output_path: S, bound: Dimensions) {
    let html_path = html_path.into();
    let output_path = output_path.into();

    let html_source = fs::read_to_string(html_path).unwrap();
    let document = html::parse(html_source);

    let mut stylesheet = css::Stylesheet::default();
    for css_path in document.collect_css_pathes() {
        let css_source = fs::read_to_string(css_path).unwrap();
        stylesheet.merge(css::parse(css_source));
    }

    let style_tree = style::style_tree(&document.root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_tree, bound);
    let display_list = display::build_display_list(&layout_root);
    pdf::render(&display_list, bound.content, output_path);
}
