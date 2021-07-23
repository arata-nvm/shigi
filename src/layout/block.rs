use crate::style::Position;

use super::{Dimensions, Keyword, LayoutBox, Length, Px};

impl<'a> LayoutBox<'a> {
    pub(crate) fn layout_block(&mut self, containing_block: Dimensions) {
        self.calculate_block_width(containing_block);

        self.calculate_block_position(containing_block);

        self.layout_block_children();

        self.calculate_block_height();
    }

    fn calculate_block_width(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();

        let auto = Keyword("auto".to_string());
        let mut width = style.value("width").unwrap_or_else(|| auto.clone());

        let zero = Length(0.0, Px);

        let mut margin_left = style.lookup("margin-left", "margin", &zero);
        let mut margin_right = style.lookup("margin-right", "margin", &zero);

        let border_left = style.lookup("border-left-width", "border-width", &zero);
        let border_right = style.lookup("border-right-width", "border-width", &zero);

        let padding_left = style.lookup("padding-left", "padding", &zero);
        let padding_right = style.lookup("padding-right", "padding", &zero);

        let total: f32 = [
            &margin_left,
            &margin_right,
            &border_left,
            &border_right,
            &padding_left,
            &padding_right,
            &width,
        ]
        .iter()
        .map(|v| v.to_px())
        .sum();

        if width != auto && total > containing_block.content.width {
            if margin_left == auto {
                margin_left = Length(0.0, Px);
            }
            if margin_right == auto {
                margin_right = Length(0.0, Px);
            }
        }

        let underflow = containing_block.content.width - total;

        match (width == auto, margin_left == auto, margin_right == auto) {
            (false, false, false) => margin_right = Length(margin_right.to_px() + underflow, Px),
            (false, false, true) => margin_left = Length(underflow, Px),
            (false, true, false) => margin_right = Length(underflow, Px),
            (false, true, true) => {
                margin_left = Length(underflow / 2.0, Px);
                margin_right = Length(underflow / 2.0, Px);
            }
            (true, _, _) => {
                if margin_left == auto {
                    margin_left = Length(0.0, Px);
                }
                if margin_right == auto {
                    margin_right = Length(0.0, Px);
                }

                if underflow >= 0.0 {
                    width = Length(underflow, Px);
                } else {
                    width = Length(0.0, Px);
                    margin_right = Length(margin_right.to_px() + underflow, Px);
                }
            }
        }

        let d = &mut self.dimensions;
        d.content.width = width.to_px();

        d.padding.left = padding_left.to_px();
        d.padding.right = padding_right.to_px();

        d.border.left = border_left.to_px();
        d.border.right = border_right.to_px();

        d.margin.left = margin_left.to_px();
        d.margin.right = margin_right.to_px();
    }

    fn calculate_block_position(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();
        let d = &mut self.dimensions;

        let zero = Length(0.0, Px);

        d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
        d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

        d.border.top = style
            .lookup("border-top-width", "border-width", &zero)
            .to_px();
        d.border.bottom = style
            .lookup("border-bottom-width", "border-width", &zero)
            .to_px();

        d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
        d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

        d.content.x = containing_block.content.x + d.margin.left + d.border.left + d.padding.left;

        d.content.y = containing_block.content.height
            + containing_block.content.y
            + d.margin.top
            + d.border.top
            + d.padding.top;

        if let Position::Relative = style.position() {
            let auto = Keyword("auto".to_string());

            let mut left = style.value_or("left", &auto);
            let right = style.value_or("right", &auto);
            let mut top = style.value_or("top", &auto);
            let bottom = style.value_or("bottom", &auto);

            match (left == auto, right == auto) {
                (true, true) => left = Length(0.0, Px),
                (true, false) => left = Length(-right.to_px(), Px),
                _ => {}
            }

            match (top == auto, bottom == auto) {
                (true, true) => top = Length(0.0, Px),
                (true, false) => top = Length(-bottom.to_px(), Px),
                _ => {}
            }

            d.content.x += left.to_px();
            d.content.y += top.to_px();
        }
    }

    fn layout_block_children(&mut self) {
        let d = &mut self.dimensions;
        for child in &mut self.children {
            child.layout(*d);
            d.content.height += child.dimensions.margin_box().height;
        }
    }

    fn calculate_block_height(&mut self) {
        if let Some(Length(h, Px)) = self.get_style_node().value("height") {
            self.dimensions.content.height = h;
        }
    }
}
