#[derive(Debug)]
pub enum CoordinateParseError {
    NotTwoNumbers,
    InvalidXValue,
    InvalidYValue,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn from_str_pair((x, y): (&str, &str)) -> Result<Self, CoordinateParseError> {
        match x.parse::<usize>() {
            Ok(x) => match y.parse::<usize>() {
                Ok(y) => Ok(Coordinate::new(x, y)),
                Err(_) => Err(CoordinateParseError::InvalidYValue),
            },
            Err(_) => Err(CoordinateParseError::InvalidXValue),
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn to_index(&self, width: usize) -> usize {
        self.y * width + self.x
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

impl std::convert::From<(usize, usize)> for Coordinate {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x, y)
    }
}

impl std::convert::From<&(usize, usize)> for Coordinate {
    fn from((x, y): &(usize, usize)) -> Self {
        Self::new(*x, *y)
    }
}

impl std::convert::From<Coordinate> for (usize, usize) {
    fn from(coordinate: Coordinate) -> Self {
        (coordinate.x, coordinate.y)
    }
}

impl std::convert::From<&Coordinate> for (usize, usize) {
    fn from(coordinate: &Coordinate) -> Self {
        (coordinate.x, coordinate.y)
    }
}
