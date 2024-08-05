//! X-Y-Z Objects



use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};



// ================================================================================================



/// An x-y-z value.
pub struct Xyz<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Xyz<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}



// ================================================================================================



impl<T: Clone> Clone for Xyz<T> {
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        }
    }
}

impl<T: Copy> Copy for Xyz<T> {}

impl<T: Debug> Debug for Xyz<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Xyz")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

impl<T: Default> Default for Xyz<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}

impl<T: Eq> Eq for Xyz<T> {}

impl<T: PartialEq> PartialEq for Xyz<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}



// ================================================================================================



impl <T: Add<Output = T>> Add<Xyz<T>> for Xyz<T> {
    type Output = Xyz<T>;

    fn add(self, rhs: Xyz<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: AddAssign> AddAssign<Xyz<T>> for Xyz<T> {
    fn add_assign(&mut self, rhs: Xyz<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl <T: Sub<Output = T>> Sub<Xyz<T>> for Xyz<T> {
    type Output = Xyz<T>;

    fn sub(self, rhs: Xyz<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: SubAssign> SubAssign<Xyz<T>> for Xyz<T> {
    fn sub_assign(&mut self, rhs: Xyz<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Xyz<f32> {
    // TODO: Check this before commiting it!
    // pub fn normalize(&self) -> Self {
    //     let v = ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
    //     Self {
    //         x: self.x / v,
    //         y: self.y / v,
    //         z: self.z / v,
    //     }
    // }
}



// ================================================================================================



#[cfg(test)]
mod tests {
    use super::*;

    /// Assert that [`Xyz`] implements Debug, Eq, Clone, etc... whenever its generic does.
    #[test]
    fn assert_implementations() {
        let u32xyz_a: Xyz<u32> = Xyz::new(1, 3, 2);
        let u32xyz_b: Xyz<u32> = Xyz::new(3, 1, 2);

        assert_eq!(u32xyz_a, u32xyz_a.clone());
        assert_ne!(u32xyz_a, u32xyz_b);
    }
}
