


use core::ops::{Add, Div, Mul, Sub};



/// A generic, 2-dimensional structure defining a value for the horizontal and vertical axes.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Xy<T> {
    /// The horizontal axis value.
    pub x: T,
    /// The vertical axis value.
    pub y: T,
}

impl<T: PartialEq> PartialEq<(T, T)> for Xy<T> {
    fn eq(&self, (x, y): &(T, T)) -> bool {
        &self.x == x && &self.y == y
    }
}

impl<T: PartialEq> PartialEq<[T; 2]> for Xy<T> {
    fn eq(&self, other: &[T; 2]) -> bool {
        self.x == other[0] && self.y == other[1]
    }
}

impl<T> Xy<T> {
    #[inline(always)]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn with_x(mut self, x: T) -> Self {
        self.x = x;
        self
    }

    #[inline]
    pub fn with_y(mut self, y: T) -> Self {
        self.y = y;
        self
    }
}

impl<T: Copy> Xy<T> {
    #[inline]
    pub const fn from_array(a: [T; 2]) -> Self {
        // NOTE: This must be Copy because Rust cannot evaluate this destructor at compile time.
        Self::new(a[0], a[1])
    }

    /// `[x, y]`
    #[inline]
    pub const fn to_array(&self) -> [T; 2] {
        [self.x, self.y]
    }

    #[inline]
    pub const fn splat(v: T) -> Self {
        Self { x: v, y: v }
    }

    #[inline]
    pub const fn pair(&self) -> (T, T) {
        (self.x, self.y)
    }
}

impl<T: PartialOrd> Xy<T> {
    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        Self {
            x: if self.x < rhs.x { self.x } else { rhs.x },
            y: if self.y < rhs.y { self.y } else { rhs.y },
        }
    }

    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        Self {
            x: if self.x > rhs.x { self.x } else { rhs.x },
            y: if self.y > rhs.y { self.y } else { rhs.y },
        }
    }
}

impl<T: Add> Xy<T> {
    #[inline]
    pub fn element_sum(self) -> T::Output {
        self.x + self.y
    }
}

impl<T: Mul> Xy<T> {
    #[inline]
    pub fn element_product(self) -> T::Output {
        self.x * self.y
    }
}



impl<T: Add<Output = T>> Add for Xy<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl<T: Sub<Output = T>> Sub for Xy<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl<T: Mul<Output = T>> Mul for Xy<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl<T: Div<Output = T>> Div for Xy<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Xy<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<&T> for Xy<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &T) -> Self {
        self.mul(*rhs)
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Xy<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

impl<T: Copy + Div<Output = T>> Div<&T> for Xy<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: &T) -> Self {
        self.div(*rhs)
    }
}



pub type Vec2 = Xy<f32>;

impl Vec2 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);

    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }

    #[inline]
    pub fn length(self) -> f32 {
        // FIXME: This uses the standard library. Maybe don't?
        self.dot(self).sqrt()
    }

    /// `1.0 / length()`
    #[inline]
    pub fn length_recip(self) -> f32 {
        self.length().recip()
    }

    #[inline]
    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length()
    }

    #[inline]
    pub fn normalize(self) -> Self {
        self.mul(self.length_recip())
    }

    #[inline]
    #[doc(alias = "mix")]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        self * (1.0 - s) + rhs * s
    }

    #[inline]
    pub fn move_towards(&self, rhs: Self, d: f32) -> Self {
        let a = rhs - *self;
        let len = a.length();
        if len <= d || len <= 1e-4 {
            return rhs;
        }
        *self + a / len * d
    }

    #[inline]
    pub fn midpoint(self, rhs: Self) -> Self {
        (self + rhs) * 0.5
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xyf32_is_vec2() {
        let a: Xy<f32> = Xy::ZERO;

        fn add_one(v: Vec2) -> Vec2 {
            v + Vec2::ONE
        }

        assert_eq!(add_one(a), Xy::ONE);
        assert_eq!(add_one(a), Vec2::ONE);
    }
}
