//! A 2-dimensional grid with runtime-defined, fixed size that allows mutable access to its fields.
#![no_std]
#![deny(unsafe_code)]
extern crate alloc;

pub use self::builder::GridBuilder;
pub use self::coordinate::Coordinate;
pub use self::errors::{BuildError, CoordinateParseError, FromIterableError};
pub use self::grid::Grid;

mod builder;
mod coordinate;
mod errors;
mod grid;
