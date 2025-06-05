//! Bog Alloc

#![no_std]



pub mod color;

pub extern crate alloc;

pub use alloc::{
    boxed::*,
    str::*,
    string::{self, String, ToString},
    sync::*,
    vec::Vec,
};
pub use color::Color;
