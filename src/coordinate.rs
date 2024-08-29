use crate::errors::CoordinateParseError;
use std::ops::Add;

/// Coordinate of a cell on a two-dimensional grid
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    // skip zero offset, which is the original coordinate
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];
const SUPPORTED_SEPARATORS: [char; 3] = ['x', ',', ' '];

impl Coordinate {
    /// Creates a new coordinate
    ///
    /// # Arguments
    ///
    /// * `x` - The x component
    /// * `y` - The y component
    ///
    #[must_use]
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Creates a coordinate from a grid's width and a total index
    ///
    /// # Arguments
    ///
    /// * `width` - The grid's width
    /// * `index` - The index of the cell
    ///
    #[must_use]
    pub const fn from_width_and_index(width: usize, index: usize) -> Self {
        let x = index % width;
        Self::new(x, (index - x) / width)
    }

    /// Returns the x component
    #[must_use]
    pub const fn x(&self) -> usize {
        self.x
    }

    /// Returns the y component
    #[must_use]
    pub const fn y(&self) -> usize {
        self.y
    }

    /// Converts the coordinate into a linear index
    ///
    /// # Arguments
    /// * `width` - The width of the grid
    ///
    #[must_use]
    pub fn to_index(&self, width: usize) -> Option<usize> {
        self.y
            .checked_mul(width)
            .and_then(|row| row.checked_add(self.x))
    }

    /// Returns all potential neighboring coordinates
    pub fn neighbors(&self) -> impl Iterator<Item = Self> + '_ {
        NEIGHBOR_OFFSETS
            .iter()
            .filter_map(move |offset| self + offset)
    }
}

impl Add<&(isize, isize)> for &Coordinate {
    type Output = Option<Coordinate>;

    fn add(self, (dx, dy): &(isize, isize)) -> Self::Output {
        self.x.checked_add_signed(*dx).and_then(|x| {
            self.y
                .checked_add_signed(*dy)
                .map(|y| Coordinate::new(x, y))
        })
    }
}

/// Create a Coordinate from a &str
///
/// # Examples
///
/// ```
/// use std::num::IntErrorKind;
/// use std::str::FromStr;
/// use grid2d::{Coordinate, CoordinateParseError};
///
/// assert!(match Coordinate::from_str("-1 1").unwrap_err() {
///     CoordinateParseError::InvalidXValue(e) => e.kind() == &IntErrorKind::InvalidDigit,
///     _ => false,
/// });
/// assert!(match Coordinate::from_str("1,-1").unwrap_err() {
///     CoordinateParseError::InvalidYValue(e) => e.kind() == &IntErrorKind::InvalidDigit,
///     _ => false,
/// });
/// assert!(match Coordinate::from_str("a 42").unwrap_err() {
///     CoordinateParseError::InvalidXValue(e) => e.kind() == &IntErrorKind::InvalidDigit,
///     _ => false,
/// });
/// assert!(match Coordinate::from_str("42xa").unwrap_err() {
///     CoordinateParseError::InvalidYValue(e) => e.kind() == &IntErrorKind::InvalidDigit,
///     _ => false,
/// });
/// assert_eq!(Coordinate::from_str("42").err(), Some(CoordinateParseError::NotTwoNumbers));
/// assert!(match Coordinate::from_str(" 42").unwrap_err() {
///     CoordinateParseError::InvalidXValue(e) => e.kind() == &IntErrorKind::Empty,
///     _ => false,
/// });
/// assert_eq!(Coordinate::from_str("abc").err(), Some(CoordinateParseError::NotTwoNumbers));
/// assert!(match Coordinate::from_str("42, ").unwrap_err() {
///     CoordinateParseError::InvalidYValue(e) => e.kind() == &IntErrorKind::Empty,
///     _ => false,
/// });
/// assert_eq!(Coordinate::from_str("42x1337").ok(), Some(Coordinate::new(42, 1337)));
/// assert_eq!(Coordinate::from_str("0, 0").ok(), Some(Coordinate::new(0, 0)));
/// ```
impl std::str::FromStr for Coordinate {
    type Err = CoordinateParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match SUPPORTED_SEPARATORS
            .into_iter()
            .find_map(|char| string.split_once(char))
        {
            Some((x, y)) => Self::try_from((x.trim(), y.trim())),
            None => Err(CoordinateParseError::NotTwoNumbers),
        }
    }
}

#[cfg(feature = "display")]
impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}

impl From<&Self> for Coordinate {
    fn from(coordinate: &Self) -> Self {
        *coordinate
    }
}

/// Create a coordinate from a (usize, usize) tuple
///
/// # Examples
///
/// ```
/// use grid2d::Coordinate;
///
/// assert_eq!(Coordinate::new(32, 1337), (32, 1337).into());
/// ```
impl From<(usize, usize)> for Coordinate {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x, y)
    }
}

/// Create a Coordinate from a reference to a (usize, usize) tuple
///
/// # Examples
///
/// ```
/// use grid2d::Coordinate;
///
/// assert_eq!(Coordinate::new(32, 1337), (&(32, 1337)).into());
/// ```
impl From<&(usize, usize)> for Coordinate {
    fn from((x, y): &(usize, usize)) -> Self {
        Self::new(*x, *y)
    }
}

/// Create a (usize, usize) tuple tuple from a Coordinate
///
/// # Examples
///
/// ```
/// use grid2d::Coordinate;
///
/// let (x, y) = Coordinate::new(32, 1337).into();
/// assert_eq!((32, 1337), (x, y));
/// ```
impl From<Coordinate> for (usize, usize) {
    fn from(coordinate: Coordinate) -> Self {
        (coordinate.x, coordinate.y)
    }
}

/// Create a (usize, usize) tuple from a Coordinate reference
///
/// # Examples
///
/// ```
/// use grid2d::Coordinate;
///
/// let (x, y) = (&Coordinate::new(32, 1337)).into();
/// assert_eq!((32, 1337), (x, y));
/// ```
impl From<&Coordinate> for (usize, usize) {
    fn from(coordinate: &Coordinate) -> Self {
        (coordinate.x, coordinate.y)
    }
}

/// Create a `Coordinate` from a `(&str, &str)` tuple.
///
/// # Examples
///
/// ```
/// use grid2d::Coordinate;
///
/// let coordinate = Coordinate::try_from(("32", "1337"));
/// assert_eq!(Ok(Coordinate::new(32, 1337)), coordinate);
/// ```
impl TryFrom<(&str, &str)> for Coordinate {
    type Error = CoordinateParseError;

    fn try_from((x, y): (&str, &str)) -> Result<Self, Self::Error> {
        x.parse::<usize>()
            .map_err(CoordinateParseError::InvalidXValue)
            .and_then(|x| {
                y.parse::<usize>()
                    .map_err(CoordinateParseError::InvalidYValue)
                    .map(|y| Self::new(x, y))
            })
    }
}
