mod errors;
pub use errors::CoordinateParseError;

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

impl Coordinate {
    /// Creates a new coordinate
    ///
    /// # Arguments
    ///
    /// * `x` - The x component
    /// * `y` - The y component
    ///
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Creates a coordinate from a grid's width and a total index
    ///
    /// # Arguments
    ///
    /// * `width` - The grid's width
    /// * `index` - The index of the cell
    ///
    pub fn from_width_and_index(width: usize, index: usize) -> Self {
        let x = index % width;
        let y = (index - x) / width;
        Self::new(x, y)
    }

    /// Creates a coordinate from a tuple of two string slices on success or returns an error code
    ///
    /// # Arguments
    ///
    /// * `(x, y)` - Tuple of the x and y component as string slices
    ///
    pub fn from_str_pair((x, y): (&str, &str)) -> Result<Self, CoordinateParseError> {
        match x.parse::<usize>() {
            Ok(x) => match y.parse::<usize>() {
                Ok(y) => Ok(Coordinate::new(x, y)),
                Err(_) => Err(CoordinateParseError::InvalidYValue),
            },
            Err(_) => Err(CoordinateParseError::InvalidXValue),
        }
    }

    /// Returns the x component
    pub fn x(&self) -> usize {
        self.x
    }

    /// Returns the y component
    pub fn y(&self) -> usize {
        self.y
    }

    /// Converts the coordinate into a linear index
    ///
    /// # Arguments
    /// * `width` - The width of the grid
    ///
    pub fn to_index(&self, width: usize) -> usize {
        self.y * width + self.x
    }

    /// Returns all potential neighboring coordinates
    pub fn neighbors(&self) -> Vec<Coordinate> {
        NEIGHBOR_OFFSETS
            .iter()
            .map(|(dx, dy)| (self.x as isize + dx, self.y as isize + dy))
            .filter(|&(x, y)| 0 <= x && 0 <= y)
            .map(|(x, y)| Self::new(x as usize, y as usize))
            .collect()
    }
}

impl std::str::FromStr for Coordinate {
    type Err = CoordinateParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.split_once(' ') {
            Some(value) => Self::from_str_pair(value),
            None => Err(CoordinateParseError::NotTwoNumbers),
        }
    }
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}

impl From<&Coordinate> for Coordinate {
    fn from(coordinate: &Coordinate) -> Self {
        *coordinate
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x, y)
    }
}

impl From<&(usize, usize)> for Coordinate {
    fn from((x, y): &(usize, usize)) -> Self {
        Self::new(*x, *y)
    }
}

impl From<Coordinate> for (usize, usize) {
    fn from(coordinate: Coordinate) -> Self {
        (coordinate.x, coordinate.y)
    }
}

impl From<&Coordinate> for (usize, usize) {
    fn from(coordinate: &Coordinate) -> Self {
        (coordinate.x, coordinate.y)
    }
}
