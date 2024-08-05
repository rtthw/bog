//! Bog
//! 
//! The highly-modular abstraction standard.



#[cfg(feature = "rect")]
pub mod rect;
#[cfg(feature = "xy")]
pub mod xy;
#[cfg(feature = "xyz")]
pub mod xyz;


pub mod prelude {
    #[cfg(feature = "rect")]
    pub use crate::rect::*;
    #[cfg(feature = "xy")]
    pub use crate::xy::*;
    #[cfg(feature = "xyz")]
    pub use crate::xyz::*;
}
