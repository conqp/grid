use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CoordinateParseError {
    NotTwoNumbers,
    InvalidXValue,
    InvalidYValue,
}

impl Display for CoordinateParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NotTwoNumbers => "not two numbers",
                Self::InvalidXValue => "invalid x value",
                Self::InvalidYValue => "invalid y value",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GridConstructionError {
    ZeroSize,
    VecSizeNotMultipleOfWidth,
}

impl Display for GridConstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ZeroSize => "width must not be zero",
                Self::VecSizeNotMultipleOfWidth => "vec size must be a multiple of width",
            }
        )
    }
}
