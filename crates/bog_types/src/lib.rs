//! Bog types

#![no_std]



#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Length {
    Px(f32),
    Em(f32),
    Lh(f32),
    Rem(f32),
    Rlh(f32),
}

#[inline]
pub const fn px(value: f32) -> Length {
    Length::Px(value)
}

#[inline]
pub const fn em(value: f32) -> Length {
    Length::Em(value)
}

#[inline]
pub const fn lh(value: f32) -> Length {
    Length::Lh(value)
}
