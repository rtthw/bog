//! Visual layouts



#[derive(Clone, Debug, Default, PartialEq)]
pub struct Layout(taffy::Style);

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

impl Layout {
    pub fn width(mut self, width: f32) -> Self {
        self.0.size.width = taffy::prelude::length(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.0.size.height = taffy::prelude::length(height);
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
}

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
}

// Display attribute.
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

impl Layout {
    pub fn align_items_center(mut self) -> Self {
        self.0.align_items = Some(taffy::AlignItems::Center);
        self
    }

    pub fn align_content_center(mut self) -> Self {
        self.0.align_content = Some(taffy::AlignContent::Center);
        self
    }

    pub fn align_self_center(mut self) -> Self {
        self.0.align_self = Some(taffy::AlignItems::Center);
        self
    }

    pub fn justify_items_center(mut self) -> Self {
        self.0.justify_items = Some(taffy::AlignItems::Center);
        self
    }

    pub fn justify_content_center(mut self) -> Self {
        self.0.justify_content = Some(taffy::AlignContent::Center);
        self
    }

    pub fn justify_self_center(mut self) -> Self {
        self.0.justify_self = Some(taffy::AlignItems::Center);
        self
    }

    pub fn align_items_start(mut self) -> Self {
        self.0.align_items = Some(taffy::AlignItems::Start);
        self
    }

    pub fn align_content_start(mut self) -> Self {
        self.0.align_content = Some(taffy::AlignContent::Start);
        self
    }

    pub fn align_self_start(mut self) -> Self {
        self.0.align_self = Some(taffy::AlignItems::Start);
        self
    }

    pub fn justify_items_start(mut self) -> Self {
        self.0.justify_items = Some(taffy::AlignItems::Start);
        self
    }

    pub fn justify_content_start(mut self) -> Self {
        self.0.justify_content = Some(taffy::AlignContent::Start);
        self
    }

    pub fn justify_self_start(mut self) -> Self {
        self.0.justify_self = Some(taffy::AlignItems::Start);
        self
    }
}
