/// Indicates errors while parsing a coordinate
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CoordinateParseError {
    NotTwoNumbers,
    InvalidXValue,
    InvalidYValue,
}

impl std::fmt::Display for CoordinateParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
