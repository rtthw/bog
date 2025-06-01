


use bog_color::Color;
use bog_style::{LineWeight, TextSlant, Unit};
use bog_types::{Length, LengthPercent, LengthPercentAuto, Percent};
use bog_window::CursorIcon;



#[derive(Clone, Debug, Default)]
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
    pub scale: Option<Percent>,
    pub gap_x: Option<LengthPercent>,
    pub gap_y: Option<LengthPercent>,

    pub inset_left: Option<LengthPercentAuto>,
    pub inset_right: Option<LengthPercentAuto>,
    pub inset_top: Option<LengthPercentAuto>,
    pub inset_bottom: Option<LengthPercentAuto>,
    pub margin_left: Option<LengthPercentAuto>,
    pub margin_right: Option<LengthPercentAuto>,
    pub margin_top: Option<LengthPercentAuto>,
    pub margin_bottom: Option<LengthPercentAuto>,
    pub padding_left: Option<LengthPercent>,
    pub padding_right: Option<LengthPercent>,
    pub padding_top: Option<LengthPercent>,
    pub padding_bottom: Option<LengthPercent>,
    pub border_left: Option<Length>,
    pub border_right: Option<Length>,
    pub border_top: Option<Length>,
    pub border_bottom: Option<Length>,

    pub font_size: Option<FontSize>,
    pub font_style: Option<TextSlant>,
    pub font_weight: Option<LineWeight>,
    pub text_align: Option<TextAlign>,

    pub color: Option<Color>,
    pub background_color: Option<Color>,

    pub align_content: Option<PositioningStyle>,
    pub align_items: Option<AlignStyle>,
    pub align_self: Option<AlignStyle>,
    pub justify_content: Option<PositioningStyle>,
    pub justify_items: Option<AlignStyle>,
    pub justify_self: Option<AlignStyle>,
}



#[derive(Clone, Copy, Debug, Default)]
pub enum DisplayStyle {
    Block,
    Flex,
    Grid,
    #[default]
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum AlignStyle {
    Start,
    End,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

#[derive(Clone, Copy, Debug)]
pub enum PositioningStyle {
    Start,
    End,
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceEvenly,
    SpaceAround,
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

#[derive(Clone, Copy, Debug, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Right,
    Center,
    Justify,
}
