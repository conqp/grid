#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CoordinateParseError {
    NotTwoNumbers,
    InvalidXValue,
    InvalidYValue,
}

impl CoordinateParseError {
    pub fn to_string(&self) -> &str {
        match self {
            Self::NotTwoNumbers => "not two numbers",
            Self::InvalidXValue => "invalid x value",
            Self::InvalidYValue => "invalid y value",
        }
    }
}
