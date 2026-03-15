use core::error::Error;
use core::fmt::{Display, Formatter};

/// Errors that can occur when building a grid from an iterable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FromIterableError {
    /// The iterable is empty.
    EmptyIterable,
    /// The amount of items is not a multiple of the specified `width`.
    SizeNotMultipleOfWidth,
}

impl Display for FromIterableError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyIterable => write!(f, "empty iterator"),
            Self::SizeNotMultipleOfWidth => write!(f, "size is not multiple of width"),
        }
    }
}

impl Error for FromIterableError {}
