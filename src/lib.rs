//! A 2-dimensional grid with runtime-defined, fixed size that allows mutable access to its fields.
#![no_std]
#![deny(unsafe_code)]
extern crate alloc;

pub use builder::GridBuilder;
pub use coordinate::Coordinate;
pub use errors::{BuildError, CoordinateParseError};
pub use grid::Grid;

mod builder;
mod coordinate;
mod errors;
mod grid;
