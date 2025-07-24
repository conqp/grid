use core::error::Error;
use core::fmt::{self, Display, Formatter};
use core::num::ParseIntError;

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
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
