


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
