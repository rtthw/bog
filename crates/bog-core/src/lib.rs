//! Bog Alloc

#![no_std]



pub extern crate alloc;

pub use alloc::{
    boxed::*,
    str::*,
    string::{self, String, ToString},
    sync::*,
    vec::Vec,
};
