use alloc::vec::Vec;
use core::error::Error;
use core::fmt::{Debug, Display, Formatter};

/// Errors that can occur when building a grid.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BuildError<T> {
    /// Neither `width` nor `height` have been set.
    NeitherWidthNotHeightSet(Vec<T>),
    /// The specified width is larger than the amount of items.
    TooWide(Vec<T>),
    /// The specified height is larger than the amount if items.
    TooTall(Vec<T>),
    /// The desired grid size (`width` * `height`) does not match the amount of items.
    SizeDoesNotMatch(Vec<T>),
    /// The amount of items is not a multiple of the specified `width`.
    SizeNotMultipleOfWidth(Vec<T>),
    /// The amount of items is not a multiple of the specified `height`.
    SizeNotMultipleOfHeight(Vec<T>),
}

impl<T> BuildError<T> {
    /// Return the items that were stored in the [`GridBuilder`](crate::GridBuilder).
    #[must_use]
    pub fn into_items(self) -> Vec<T> {
        match self {
            Self::NeitherWidthNotHeightSet(items)
            | Self::TooWide(items)
            | Self::TooTall(items)
            | Self::SizeDoesNotMatch(items)
            | Self::SizeNotMultipleOfWidth(items)
            | Self::SizeNotMultipleOfHeight(items) => items,
        }
    }
}

impl<T> Display for BuildError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NeitherWidthNotHeightSet(_) => {
                write!(f, "neither a width nor a height was set")
            }
            Self::TooWide(_) => write!(f, "the specified width exceeds the amount of items"),
            Self::TooTall(_) => write!(f, "the specified height exceeds the amount of items"),
            Self::SizeDoesNotMatch(_) => {
                write!(f, "the desired size does not match the amount of items")
            }
            Self::SizeNotMultipleOfWidth(_) => {
                write!(
                    f,
                    "the desired width is not a multiple of the amount of items"
                )
            }
            Self::SizeNotMultipleOfHeight(_) => {
                write!(
                    f,
                    "the desired height is not a multiple of the amount of items"
                )
            }
        }
    }
}

impl<T> Error for BuildError<T> where T: Debug {}
