//! Style type



use bog_core::Color;



/// Element styling.
pub struct Style {
    pub visual: VisualStyle,
    pub layout: LayoutStyle,
}

/// Styling that does not affect placement.
pub struct VisualStyle {
    pub opacity: f32,
    pub foreground_color: Color,
    pub background_color: Color,
    pub border_color: Color,
    pub border_radius: [f32; 4],
    pub shadow_color: Color,
    pub shadow_spread: f32,
}

/// Styling that affects placement.
pub struct LayoutStyle(pub(crate) taffy::Style);



// ---



impl taffy::CoreStyle for LayoutStyle {
    #[inline(always)]
    fn box_generation_mode(&self) -> taffy::BoxGenerationMode {
        match self.0.display {
            taffy::Display::None => taffy::BoxGenerationMode::None,
            _ => taffy::BoxGenerationMode::Normal,
        }
    }
    #[inline(always)]
    fn is_block(&self) -> bool {
        matches!(self.0.display, taffy::Display::Block)
    }
    #[inline(always)]
    fn is_compressible_replaced(&self) -> bool {
        self.0.item_is_replaced
    }
    #[inline(always)]
    fn box_sizing(&self) -> taffy::BoxSizing {
        self.0.box_sizing
    }
    #[inline(always)]
    fn overflow(&self) -> taffy::Point<taffy::Overflow> {
        self.0.overflow
    }
    #[inline(always)]
    fn scrollbar_width(&self) -> f32 {
        self.0.scrollbar_width
    }
    #[inline(always)]
    fn position(&self) -> taffy::Position {
        self.0.position
    }
    #[inline(always)]
    fn inset(&self) -> taffy::Rect<taffy::LengthPercentageAuto> {
        self.0.inset
    }
    #[inline(always)]
    fn size(&self) -> taffy::Size<taffy::Dimension> {
        self.0.size
    }
    #[inline(always)]
    fn min_size(&self) -> taffy::Size<taffy::Dimension> {
        self.0.min_size
    }
    #[inline(always)]
    fn max_size(&self) -> taffy::Size<taffy::Dimension> {
        self.0.max_size
    }
    #[inline(always)]
    fn aspect_ratio(&self) -> Option<f32> {
        self.0.aspect_ratio
    }
    #[inline(always)]
    fn margin(&self) -> taffy::Rect<taffy::LengthPercentageAuto> {
        self.0.margin
    }
    #[inline(always)]
    fn padding(&self) -> taffy::Rect<taffy::LengthPercentage> {
        self.0.padding
    }
    #[inline(always)]
    fn border(&self) -> taffy::Rect<taffy::LengthPercentage> {
        self.0.border
    }
}
