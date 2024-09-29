use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

/// Errors that can occur when parsing a coordinate from a string.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CoordinateParseError {
    /// The string does not contain two numbers for x and y.
    NotTwoNumbers,
    /// The value for the x coordinate is invalid.
    InvalidXValue(ParseIntError),
    /// The value for the y coordinate is invalid.
    InvalidYValue(ParseIntError),
}

impl Display for CoordinateParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotTwoNumbers => write!(f, "not two numbers"),
            Self::InvalidXValue(error) => write!(f, "invalid x value: {error}"),
            Self::InvalidYValue(error) => write!(f, "invalid y value: {error}"),
        }
    }
}

impl Error for CoordinateParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotTwoNumbers => None,
            Self::InvalidXValue(error) | Self::InvalidYValue(error) => Some(error),
        }
    }
}

/// An error that can occur when constructing a Grid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GridConstructionError {
    /// The size of the passed-in Vec is not a non-zero multiple of the grid's width.
    VecSizeNotMultipleOfWidth,
}

impl Display for GridConstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::VecSizeNotMultipleOfWidth => "vec size must be a multiple of width",
            }
        )
    }
}

impl Error for GridConstructionError {}
