//! A 2-dimensional grid with runtime-defined, fixed size that allows mutable access to its fields.
#![no_std]
#![deny(unsafe_code)]
extern crate alloc;

pub use coordinate::Coordinate;
pub use errors::CoordinateParseError;
pub use grid::Grid;

mod coordinate;
mod errors;
mod grid;
