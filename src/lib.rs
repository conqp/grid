//! A 2-dimensional grid with runtime-defined, fixed size that allows mutable access to its fields.

pub use coordinate::Coordinate;
pub use errors::CoordinateParseError;
pub use grid::Grid;

mod coordinate;
mod errors;
mod grid;
