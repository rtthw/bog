//! Bog core types

#![no_std]



mod color;
mod event;
mod input;
mod key;
mod nohash_map;
mod rect;
mod type_map;
mod unit_map;


pub extern crate alloc;

pub use alloc::{
    boxed::*,
    str::*,
    string::{self, String, ToString},
    sync::*,
    vec::Vec,
};

#[macro_export]
macro_rules! vec {
    () => (
        $crate::alloc::vec::Vec::new()
    );
    ($elem:expr; $n:expr) => (
        $crate::alloc::vec::from_elem($elem, $n)
    );
    ($($x:expr),+ $(,)?) => (
        <[_]>::into_vec(
            // NOTE: This doesn't use the `box_new` intrinsic because it isn't available on stable.
            //       Using the intrinsic produces a dramatic improvement in stack usage for
            //       unoptimized programs using this code path to construct large Vecs.
            $crate::alloc::boxed::Box::new([$($x),+])
        )
    );
}

pub use color::Color;
pub use event::{InputEvent, MouseButton, WheelMovement, WindowEvent};
pub use input::{Input, InputHandler};
pub use key::{KeyCode, KeyUpdate};
pub use nohash_map::NoHashMap;
pub use rect::Rect;
pub use type_map::TypeMap;
pub use unit_map::UnitMap;


// ---


pub type Vec2 = glam::Vec2;
pub type Vec3 = glam::Vec3;
pub type Vec4 = glam::Vec4;



pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4::new(x, y, z, w)
}



pub type Mat2 = glam::Mat2;
pub type Mat3 = glam::Mat3;
pub type Mat4 = glam::Mat4;



pub fn mat3_translation(translation: Vec2) -> Mat3 {
    Mat3::from_translation(translation)
}

pub fn mat4_translation(translation: Vec3) -> Mat4 {
    Mat4::from_translation(translation)
}

pub fn mat3_scale(scale: Vec2) -> Mat3 {
    Mat3::from_scale(scale)
}

pub fn mat4_scale(scale: Vec3) -> Mat4 {
    Mat4::from_scale(scale)
}
