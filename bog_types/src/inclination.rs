//! Inclination Type



/// The tendency toward something spatially or temporally.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Inclination {
    #[default]
    Up,
    Down,
    Left,
    Right,

    Forward,
    Backward,

    Top,
    Middle,
    Bottom,

    First,
    Last,
    Previous,
    Next,
}

// Checks.
impl Inclination {
    /// Whether this inclination is aimed at one of the four directions.
    pub fn is_directional(&self) -> bool {
        matches!(self, Self::Up | Self::Down | Self::Left | Self::Right)
    }

    /// Whether this inclination is positional (relating to some ordered set).
    pub fn is_positional(&self) -> bool {
        matches!(
            self,
            Self::Previous | Self::Next |
            Self::First | Self::Last |
            Self::Forward | Self::Backward |
            Self::Middle
        )
    }

    /// Whether this inclination is spatial (relating to physical space).
    pub fn is_spatial(&self) -> bool {
        matches!(
            self,
            Self::Up | Self::Down | Self::Left | Self::Right |
            Self::Forward | Self::Backward |
            Self::Top | Self::Middle | Self::Bottom
        )
    }

    /// Whether this inclination is temporal (relating to time).
    pub fn is_temporal(&self) -> bool {
        matches!(
            self,
            Self::Forward | Self::Backward |
            Self::Previous | Self::Next |
            Self::First | Self::Last |
            Self::Middle
        )
    }
}

// Conversions.
impl Inclination {
    /// Convert this inclination to its directional counterpart, if it's not already.
    pub fn to_directional(self) -> Self {
        match self {
            Self::Forward => Self::Right,
            Self::Backward => Self::Left,
            Self::Top => Self::Up,
            Self::Middle => Self::Right, // Is this is right?
            Self::Bottom => Self::Down,
            Self::First => Self::Left,
            Self::Last => Self::Right,
            Self::Previous => Self::Left,
            Self::Next => Self::Right,

            i => i,
        }
    }

    /// Convert this inclination to its positional counterpart, if it's not already.
    pub fn to_positional(self) -> Self {
        match self {
            Self::Up => Self::Forward,
            Self::Down => Self::Backward,
            Self::Left => Self::Previous,
            Self::Right => Self::Next,
            Self::Top => Self::First,
            Self::Bottom => Self::Last,

            i => i,
        }
    }

    /// Convert this inclination to its spatial counterpart, if it's not already.
    pub fn to_spatial(self) -> Self {
        match self {
            Self::First => Self::Top,
            Self::Last => Self::Bottom,
            Self::Previous => Self::Left,
            Self::Next => Self::Right,

            i => i,
        }
    }

    /// Convert this inclination to its temporal counterpart, if it's not already.
    pub fn to_temporal(self) -> Self {
        match self {
            Self::Up => Self::Forward,
            Self::Down => Self::Backward,
            Self::Left => Self::Previous,
            Self::Right => Self::Next,
            Self::Top => Self::First,
            Self::Bottom => Self::Last,

            i => i,
        }
    }
}
