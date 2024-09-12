use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CoordinateParseError {
    NotTwoNumbers,
    InvalidXValue(ParseIntError),
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GridConstructionError {
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
