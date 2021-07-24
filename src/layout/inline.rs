use crate::{html::NodeType, text::calc_text_region};

use super::{BoxType, Dimensions, LayoutBox, Length, Px};

impl<'a> LayoutBox<'a> {
    pub(crate) fn layout_inline(&mut self, containing_block: Dimensions) {
        self.calculate_width();

        self.calculate_dimentions();

        self.calculate_position(containing_block);

        self.layout_children();
    }

    fn calculate_width(&mut self) {
        match self.box_type {
            BoxType::InlineNode(ref style) => match style.node.typ {
                NodeType::Text(ref text) => {
                    let region = calc_text_region(text.clone());
                    self.dimensions.content.width = region.width;
                    self.dimensions.content.height = region.height;
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn calculate_dimentions(&mut self) {
        let style = self.get_style_node();
        let zero = Length(0.0, Px);

        let d = &mut self.dimensions;

        d.padding.left = style.lookup("padding-left", "padding", &zero).to_px();
        d.padding.right = style.lookup("padding-right", "padding", &zero).to_px();
        d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
        d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

        d.border.left = style
            .lookup("border-left-width", "border-width", &zero)
            .to_px();
        d.border.right = style
            .lookup("border-right-width", "border-width", &zero)
            .to_px();
        d.border.top = style
            .lookup("border-top-width", "border-width", &zero)
            .to_px();
        d.border.bottom = style
            .lookup("border-bottom-width", "border-width", &zero)
            .to_px();

        d.margin.left = style.lookup("margin-left", "margin", &zero).to_px();
        d.margin.right = style.lookup("margin-right", "margin", &zero).to_px();
    }

    fn calculate_position(&mut self, containing_block: Dimensions) {
        let d = &mut self.dimensions;

        d.content.x = containing_block.content.x
            + containing_block.content.width
            + d.padding.left
            + d.border.left
            + d.margin.left;
        d.content.y = containing_block.content.y + containing_block.content.height;
    }

    fn layout_children(&mut self) {
        let mut max_height = 0.0f32;

        for child in &mut self.children {
            child.layout(self.dimensions);

            self.dimensions.content.width += child.dimensions.margin_box().width;
            max_height = max_height.max(child.dimensions.content.height);
        }

        self.dimensions.content.height = max_height.max(self.dimensions.content.height);
    }

    pub(crate) fn layout_anonymous_block(&mut self, mut containing_block: Dimensions) {
        let mut max_height = 0.0f32;
        containing_block.content.width = 0.0;

        for child in &mut self.children {
            child.layout(containing_block);

            containing_block.content.width += child.dimensions.margin_box().width;
            max_height = max_height.max(child.dimensions.content.height);
        }

        self.dimensions.content.height = max_height;
    }
}
