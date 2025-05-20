//! Layout type



#[derive(Clone, Debug, Default, PartialEq)]
pub struct Layout(pub(crate) taffy::Style);

impl From<taffy::Style> for Layout {
    fn from(value: taffy::Style) -> Self {
        Self(value)
    }
}

impl Into<taffy::Style> for Layout {
    fn into(self) -> taffy::Style {
        self.0
    }
}

// Sizing.
impl Layout {
    pub fn width(mut self, width: f32) -> Self {
        self.0.size.width = taffy::prelude::length(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.0.size.height = taffy::prelude::length(height);
        self
    }

    pub fn width_percent(mut self, width: f32) -> Self {
        self.0.size.width = taffy::prelude::percent(width);
        self
    }

    pub fn height_percent(mut self, height: f32) -> Self {
        self.0.size.height = taffy::prelude::percent(height);
        self
    }

    pub fn fill_width(mut self) -> Self {
        self.0.size.width = taffy::prelude::percent(1.0);
        self
    }

    pub fn fill_height(mut self) -> Self {
        self.0.size.height = taffy::prelude::percent(1.0);
        self
    }

    pub fn width_auto(mut self) -> Self {
        self.0.size.width = taffy::prelude::auto();
        self
    }

    pub fn height_auto(mut self) -> Self {
        self.0.size.height = taffy::prelude::auto();
        self
    }

    pub fn get_width(&self) -> Option<f32> {
        self.0.size.width.into_option()
    }

    pub fn get_height(&self) -> Option<f32> {
        self.0.size.height.into_option()
    }
}

// Margin, spacing, and padding.
impl Layout {
    pub fn gap_x(mut self, width: f32) -> Self {
        self.0.gap.width = taffy::LengthPercentage::Length(width);
        self
    }

    pub fn gap_y(mut self, height: f32) -> Self {
        self.0.gap.height = taffy::LengthPercentage::Length(height);
        self
    }

    pub fn padding(mut self, amount: f32) -> Self {
        self.0.padding.left = taffy::LengthPercentage::Length(amount);
        self.0.padding.right = taffy::LengthPercentage::Length(amount);
        self.0.padding.top = taffy::LengthPercentage::Length(amount);
        self.0.padding.bottom = taffy::LengthPercentage::Length(amount);
        self
    }

    pub fn margin(mut self, amount: f32) -> Self {
        self.0.margin.left = taffy::LengthPercentageAuto::Length(amount);
        self.0.margin.right = taffy::LengthPercentageAuto::Length(amount);
        self.0.margin.top = taffy::LengthPercentageAuto::Length(amount);
        self.0.margin.bottom = taffy::LengthPercentageAuto::Length(amount);
        self
    }

    pub fn margin_auto(mut self) -> Self {
        self.0.margin.left = taffy::LengthPercentageAuto::Auto;
        self.0.margin.right = taffy::LengthPercentageAuto::Auto;
        self.0.margin.top = taffy::LengthPercentageAuto::Auto;
        self.0.margin.bottom = taffy::LengthPercentageAuto::Auto;
        self
    }
}

// Display.
impl Layout {
    pub fn display_none(mut self) -> Self {
        self.0.display = taffy::Display::None;
        self
    }

    pub fn display_block(mut self) -> Self {
        self.0.display = taffy::Display::Block;
        self
    }

    pub fn display_grid(mut self) -> Self {
        self.0.display = taffy::Display::Grid;
        self
    }

    pub fn display_flex(mut self) -> Self {
        self.0.display = taffy::Display::Flex;
        self
    }

    pub fn overflow_visible_y(mut self) -> Self {
        self.0.overflow.y = taffy::Overflow::Visible;
        self
    }

    pub fn overflow_scroll_y(mut self) -> Self {
        self.0.overflow.y = taffy::Overflow::Scroll;
        self
    }

    pub fn overflow_clip_y(mut self) -> Self {
        self.0.overflow.y = taffy::Overflow::Clip;
        self
    }

    pub fn overflow_hide_y(mut self) -> Self {
        self.0.overflow.y = taffy::Overflow::Hidden;
        self
    }

    pub fn overflow_visible_x(mut self) -> Self {
        self.0.overflow.x = taffy::Overflow::Visible;
        self
    }

    pub fn overflow_scroll_x(mut self) -> Self {
        self.0.overflow.x = taffy::Overflow::Scroll;
        self
    }

    pub fn overflow_clip_x(mut self) -> Self {
        self.0.overflow.x = taffy::Overflow::Clip;
        self
    }

    pub fn overflow_hide_x(mut self) -> Self {
        self.0.overflow.x = taffy::Overflow::Hidden;
        self
    }
}

// Flex.
impl Layout {
    pub fn flex_grow(mut self, flex_grow: f32) -> Self {
        self.0.flex_grow = flex_grow;
        self
    }

    pub fn flex_shrink(mut self, flex_shrink: f32) -> Self {
        self.0.flex_shrink = flex_shrink;
        self
    }

    pub fn flex_basis_len(mut self, len: f32) -> Self {
        self.0.flex_basis = taffy::Dimension::Length(len);
        self
    }

    pub fn flex_basis_percent(mut self, percent: f32) -> Self {
        self.0.flex_basis = taffy::Dimension::Percent(percent);
        self
    }

    pub fn flex_none(mut self) -> Self {
        self.0.flex_grow = 0.0;
        self.0.flex_shrink = 0.0;
        self.0.flex_basis = taffy::Dimension::Auto;
        self
    }

    pub fn flex_auto(mut self) -> Self {
        self.0.flex_grow = 1.0;
        self.0.flex_shrink = 1.0;
        self.0.flex_basis = taffy::Dimension::Auto;
        self
    }

    pub fn flex_initial(mut self) -> Self {
        self.0.flex_grow = 0.0;
        self.0.flex_shrink = 1.0;
        self.0.flex_basis = taffy::Dimension::Auto;
        self
    }

    pub fn flex_row(mut self) -> Self {
        self.0.flex_direction = taffy::FlexDirection::Row;
        self
    }

    pub fn flex_column(mut self) -> Self {
        self.0.flex_direction = taffy::FlexDirection::Column;
        self
    }

    pub fn flex_row_reverse(mut self) -> Self {
        self.0.flex_direction = taffy::FlexDirection::RowReverse;
        self
    }

    pub fn flex_column_reverse(mut self) -> Self {
        self.0.flex_direction = taffy::FlexDirection::ColumnReverse;
        self
    }

    pub fn flex_wrap(mut self) -> Self {
        self.0.flex_wrap = taffy::FlexWrap::Wrap;
        self
    }

    pub fn flex_nowrap(mut self) -> Self {
        self.0.flex_wrap = taffy::FlexWrap::NoWrap;
        self
    }

    pub fn flex_wrap_reverse(mut self) -> Self {
        self.0.flex_wrap = taffy::FlexWrap::WrapReverse;
        self
    }
}

// Align self.
impl Layout {
    pub fn align_self_start(mut self) -> Self {
        self.0.align_self = Some(taffy::AlignItems::Start);
        self
    }

    pub fn align_self_end(mut self) -> Self {
        self.0.align_self = Some(taffy::AlignItems::End);
        self
    }

    pub fn align_self_flex_start(mut self) -> Self {
        self.0.align_self = Some(taffy::AlignItems::FlexStart);
        self
    }

    pub fn align_self_flex_end(mut self) -> Self {
        self.0.align_self = Some(taffy::AlignItems::FlexEnd);
        self
    }

    pub fn align_self_center(mut self) -> Self {
        self.0.align_self = Some(taffy::AlignItems::Center);
        self
    }

    pub fn align_self_stretch(mut self) -> Self {
        self.0.align_self = Some(taffy::AlignItems::Stretch);
        self
    }

    pub fn align_self_baseline(mut self) -> Self {
        self.0.align_self = Some(taffy::AlignItems::Baseline);
        self
    }
}

// Align items.
impl Layout {
    pub fn align_items_start(mut self) -> Self {
        self.0.align_items = Some(taffy::AlignItems::Start);
        self
    }

    pub fn align_items_end(mut self) -> Self {
        self.0.align_items = Some(taffy::AlignItems::End);
        self
    }

    pub fn align_items_flex_start(mut self) -> Self {
        self.0.align_items = Some(taffy::AlignItems::FlexStart);
        self
    }

    pub fn align_items_flex_end(mut self) -> Self {
        self.0.align_items = Some(taffy::AlignItems::FlexEnd);
        self
    }

    pub fn align_items_center(mut self) -> Self {
        self.0.align_items = Some(taffy::AlignItems::Center);
        self
    }

    pub fn align_items_stretch(mut self) -> Self {
        self.0.align_items = Some(taffy::AlignItems::Stretch);
        self
    }

    pub fn align_items_baseline(mut self) -> Self {
        self.0.align_items = Some(taffy::AlignItems::Baseline);
        self
    }
}

// Align content.
impl Layout {
    pub fn align_content_start(mut self) -> Self {
        self.0.align_content = Some(taffy::AlignContent::Start);
        self
    }

    pub fn align_content_end(mut self) -> Self {
        self.0.align_content = Some(taffy::AlignContent::End);
        self
    }

    pub fn align_content_flex_start(mut self) -> Self {
        self.0.align_content = Some(taffy::AlignContent::FlexStart);
        self
    }

    pub fn align_content_flex_end(mut self) -> Self {
        self.0.align_content = Some(taffy::AlignContent::FlexEnd);
        self
    }

    pub fn align_content_center(mut self) -> Self {
        self.0.align_content = Some(taffy::AlignContent::Center);
        self
    }

    pub fn align_content_stretch(mut self) -> Self {
        self.0.align_content = Some(taffy::AlignContent::Stretch);
        self
    }

    pub fn align_content_space_evenly(mut self) -> Self {
        self.0.align_content = Some(taffy::AlignContent::SpaceEvenly);
        self
    }

    pub fn align_content_space_around(mut self) -> Self {
        self.0.align_content = Some(taffy::AlignContent::SpaceAround);
        self
    }
}

// Justify self.
impl Layout {
    pub fn justify_self_start(mut self) -> Self {
        self.0.justify_self = Some(taffy::AlignItems::Start);
        self
    }

    pub fn justify_self_end(mut self) -> Self {
        self.0.justify_self = Some(taffy::AlignItems::End);
        self
    }

    pub fn justify_self_flex_start(mut self) -> Self {
        self.0.justify_self = Some(taffy::AlignItems::FlexStart);
        self
    }

    pub fn justify_self_flex_end(mut self) -> Self {
        self.0.justify_self = Some(taffy::AlignItems::FlexEnd);
        self
    }

    pub fn justify_self_center(mut self) -> Self {
        self.0.justify_self = Some(taffy::AlignItems::Center);
        self
    }

    pub fn justify_self_stretch(mut self) -> Self {
        self.0.justify_self = Some(taffy::AlignItems::Stretch);
        self
    }

    pub fn justify_self_baseline(mut self) -> Self {
        self.0.justify_self = Some(taffy::AlignItems::Baseline);
        self
    }
}

// Justify items.
impl Layout {
    pub fn justify_items_start(mut self) -> Self {
        self.0.justify_items = Some(taffy::AlignItems::Start);
        self
    }

    pub fn justify_items_end(mut self) -> Self {
        self.0.justify_items = Some(taffy::AlignItems::End);
        self
    }

    pub fn justify_items_flex_start(mut self) -> Self {
        self.0.justify_items = Some(taffy::AlignItems::FlexStart);
        self
    }

    pub fn justify_items_flex_end(mut self) -> Self {
        self.0.justify_items = Some(taffy::AlignItems::FlexEnd);
        self
    }

    pub fn justify_items_center(mut self) -> Self {
        self.0.justify_items = Some(taffy::AlignItems::Center);
        self
    }

    pub fn justify_items_stretch(mut self) -> Self {
        self.0.justify_items = Some(taffy::AlignItems::Stretch);
        self
    }

    pub fn justify_items_baseline(mut self) -> Self {
        self.0.justify_items = Some(taffy::AlignItems::Baseline);
        self
    }
}

// Justify content.
impl Layout {
    pub fn justify_content_start(mut self) -> Self {
        self.0.justify_content = Some(taffy::AlignContent::Start);
        self
    }

    pub fn justify_content_end(mut self) -> Self {
        self.0.justify_content = Some(taffy::AlignContent::End);
        self
    }

    pub fn justify_content_flex_start(mut self) -> Self {
        self.0.justify_content = Some(taffy::AlignContent::FlexStart);
        self
    }

    pub fn justify_content_flex_end(mut self) -> Self {
        self.0.justify_content = Some(taffy::AlignContent::FlexEnd);
        self
    }

    pub fn justify_content_center(mut self) -> Self {
        self.0.justify_content = Some(taffy::AlignContent::Center);
        self
    }

    pub fn justify_content_stretch(mut self) -> Self {
        self.0.justify_content = Some(taffy::AlignContent::Stretch);
        self
    }

    pub fn justify_content_space_evenly(mut self) -> Self {
        self.0.justify_content = Some(taffy::AlignContent::SpaceEvenly);
        self
    }

    pub fn justify_content_space_around(mut self) -> Self {
        self.0.justify_content = Some(taffy::AlignContent::SpaceAround);
        self
    }
}
