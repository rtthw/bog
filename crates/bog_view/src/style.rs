


use bog_color::Color;
use bog_style::{LineWeight, TextSlant, Unit};
use bog_window::CursorIcon;



#[derive(Clone, Debug)]
pub struct ElementStyle {
    pub cursor: Option<CursorIcon>,
    pub display: Option<DisplayStyle>,
    pub opacity: Option<f32>,
    pub aspect_ratio: Option<f32>,

    pub width: Option<Unit>,
    pub height: Option<Unit>,
    pub min_width: Option<Unit>,
    pub min_height: Option<Unit>,
    pub max_width: Option<Unit>,
    pub max_height: Option<Unit>,
    pub scale: Option<Unit>,
    pub gap_x: Option<Unit>,
    pub gap_y: Option<Unit>,
    pub inset_left: Option<Unit>,
    pub inset_right: Option<Unit>,
    pub inset_top: Option<Unit>,
    pub inset_bottom: Option<Unit>,
    pub margin_left: Option<Unit>,
    pub margin_right: Option<Unit>,
    pub margin_top: Option<Unit>,
    pub margin_bottom: Option<Unit>,

    pub font_size: Option<FontSize>,
    pub font_style: Option<TextSlant>,
    pub font_weight: Option<LineWeight>,

    pub background_color: Option<Color>,
}



#[derive(Clone, Copy, Debug, Default)]
pub enum DisplayStyle {
    Block,
    Flex,
    Grid,
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum FontSize {
    Small,
    #[default]
    Medium,
    Large,
    Length(Unit),
    Percent(f32),
}
