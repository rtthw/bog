//! Rectangle Objects



use std::fmt::Debug;

use crate::prelude::Xy;



// ================================================================================================



pub struct Rect<T> {
    pub min: Xy<T>,
    pub max: Xy<T>,
}

impl <T> Rect<T> {
    pub fn new(min_x: T, min_y: T, max_x: T, max_y: T) -> Self {
        Self {
            min: Xy::new(min_x, min_y),
            max: Xy::new(max_x, max_y),
        }
    }
}

impl<T: PartialOrd> Rect<T> {
    pub fn intersects(&self, other: &Self) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
    }

    pub fn contains(self, xy: Xy<T>) -> bool {
        xy.x >= self.min.x
            && xy.x < self.max.x
            && xy.y >= self.min.y
            && xy.y < self.max.y
    }
}

impl<T: Copy> Rect<T> {
    pub fn left(&self) -> T {
        self.min.x
    }

    pub fn right(&self) -> T {
        self.max.x
    }

    pub fn top(&self) -> T {
        self.min.y
    }
    
    pub fn bottom(&self) -> T {
        self.max.y
    }
}



// ================================================================================================



impl<T: Clone> Clone for Rect<T> {
    fn clone(&self) -> Self {
        Self {
            min: self.min.clone(),
            max: self.max.clone(),
        }
    }
}

impl<T: Copy> Copy for Rect<T> {}

impl<T: Debug> Debug for Rect<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rect")
            .field("min", &self.min)
            .field("max", &self.max)
            .finish()
    }
}

impl<T: Default> Default for Rect<T> {
    fn default() -> Self {
        Self {
            min: Xy::default(),
            max: Xy::default(),
        }
    }
}

impl<T: Eq> Eq for Rect<T> {}

impl<T: PartialEq> PartialEq for Rect<T> {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}



// ================================================================================================



#[cfg(test)]
mod tests {
    use super::*;

    /// Assert that [`Rect`] implements Debug, Eq, Clone, etc... whenever its generic does.
    #[test]
    fn assert_implementations() {
        let u32rect_a: Rect<u32> = Rect::new(1, 3, 2, 4);
        let u32rect_b: Rect<u32> = Rect::new(3, 1, 4, 2);

        assert_eq!(u32rect_a, u32rect_a.clone());
        assert_ne!(u32rect_a, u32rect_b);
    }
}
