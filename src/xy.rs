//! X-Y Objects



use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};



/// An x-y value.
pub struct Xy<T> {
    pub x: T,
    pub y: T,
}

impl<T> Xy<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Copy> Xy<T> {
    #[inline]
    pub const fn swapped(&self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }
}



impl<T: Clone> Clone for Xy<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T: Copy> Copy for Xy<T> {}

impl<T: Debug> Debug for Xy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Xy").field("x", &self.x).field("y", &self.y).finish()
    }
}

impl<T: Default> Default for Xy<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T: Eq> Eq for Xy<T> {}

impl<T: PartialEq> PartialEq for Xy<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}



impl <T: Add<Output = T>> Add<Xy<T>> for Xy<T> {
    type Output = Xy<T>;

    fn add(self, rhs: Xy<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign<Xy<T>> for Xy<T> {
    fn add_assign(&mut self, rhs: Xy<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl <T: Sub<Output = T>> Sub<Xy<T>> for Xy<T> {
    type Output = Xy<T>;

    fn sub(self, rhs: Xy<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign<Xy<T>> for Xy<T> {
    fn sub_assign(&mut self, rhs: Xy<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Xy<f32> {
    // TODO: Check this before commiting it!
    // pub fn normalize(&self) -> Self {
    //     let v = ((self.x * self.x) + (self.y * self.y)).sqrt();
    //     Self {
    //         x: self.x / v,
    //         y: self.y / v,
    //     }
    // }
}



#[cfg(test)]
mod tests {
    use super::*;

    /// Assert that [`Xy`] implements Debug, Eq, Clone, etc... whenever its generic does.
    #[test]
    fn assert_implementations() {
        let u32xy_a: Xy<u32> = Xy::new(1, 3);
        let u32xy_b: Xy<u32> = Xy::new(3, 1);

        assert_eq!(u32xy_a, u32xy_a.clone());
        assert_ne!(u32xy_a, u32xy_b);
    }

    #[test]
    fn simple_use_case() {
        type Position = Xy<u16>;

        trait PositionImpl {
            fn custom_x_getter(&self) -> u16;
        }

        impl PositionImpl for Position {
            fn custom_x_getter(&self) -> u16 {
                self.x
            }
        }

        let pos_a = Position::new(1, 2);

        assert_eq!(pos_a.custom_x_getter(), 1);
    }
}
