pub mod pdf;

use crate::css::{Color, Unit::*, Value};
use crate::html::NodeType;
use crate::layout::{BoxType, LayoutBox, Position, Rect};

pub type DisplayList = Vec<DisplayCommand>;

#[derive(Debug)]
pub enum DisplayCommand {
    SolidColor(Color, Rect),
    Text(String, Position, f32),
}

pub fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    return list;
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    // debug_draw(list, layout_box);

    render_background(list, layout_box);
    render_borders(list, layout_box);
    render_text(list, layout_box);

    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

fn debug_draw(list: &mut DisplayList, layout_box: &LayoutBox) {
    list.push(DisplayCommand::SolidColor(
        Color::new(255, 0, 0, 30),
        layout_box.dimensions.margin_box(),
    ));
    list.push(DisplayCommand::SolidColor(
        Color::new(0, 0, 255, 30),
        layout_box.dimensions.padding_box(),
    ));
    list.push(DisplayCommand::SolidColor(
        Color::new(0, 255, 0, 255),
        Rect {
            width: 1.0,
            height: 1.0,
            ..layout_box.dimensions.content
        },
    ));
}

fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    get_color(layout_box, "background").map(|color| {
        list.push(DisplayCommand::SolidColor(
            color,
            layout_box.dimensions.border_box(),
        ))
    });
}

fn get_color(layout_box: &LayoutBox, name: &str) -> Option<Color> {
    match layout_box.box_type {
        BoxType::BlockNode(style) | BoxType::InlineNode(style) => match style.value(name) {
            Some(Value::ColorValue(color)) => Some(color),
            _ => None,
        },
        BoxType::AnonymousBlock => None,
    }
}

fn render_borders(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "border-color") {
        Some(color) => color,
        _ => return,
    };

    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x,
            y: border_box.y,
            width: d.border.left,
            height: border_box.height,
        },
    ));

    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x + border_box.width - d.border.left,
            y: border_box.y,
            width: d.border.right,
            height: border_box.height,
        },
    ));

    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x,
            y: border_box.y,
            width: border_box.width,
            height: d.border.top,
        },
    ));

    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x,
            y: border_box.y + border_box.height - d.border.bottom,
            width: border_box.width,
            height: d.border.bottom,
        },
    ))
}

fn render_text(list: &mut DisplayList, layout_box: &LayoutBox) {
    match layout_box.box_type {
        BoxType::InlineNode(ref style) => match style.node.typ {
            NodeType::Text(ref text) => {
                let pos = layout_box.dimensions.content;
                let size = style
                    .value_or("font-size", &Value::Length(16.0, Px))
                    .to_px();
                list.push(DisplayCommand::Text(
                    text.clone(),
                    Position::new(pos.x, pos.y),
                    size,
                ));
            }
            _ => {}
        },
        _ => {}
    }
}
