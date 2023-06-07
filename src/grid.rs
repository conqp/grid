use crate::coordinate::Coordinate;
use crate::error::GridConstructionError;
use itertools::Itertools;
use std::fmt::{Display, Formatter};

/// A two-dimensional grid of arbitrary cell content
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid<T> {
    width: usize,
    items: Vec<T>,
}

impl<T> Grid<T> {
    /// Returns a new instance of Grid
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the grid
    /// * `height` - The height of the grid
    /// * `initializer` - A function that takes no arguments and returns an instance of the cell type
    ///
    pub fn new(width: usize, height: usize, initializer: impl Fn() -> T) -> Self {
        Self {
            width,
            items: (0..width * height).map(|_| initializer()).collect(),
        }
    }

    /// Returns the width of the grid
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid
    pub fn height(&self) -> usize {
        if self.width == 0 {
            0
        } else {
            self.items.len() / self.width
        }
    }

    /// Returns the size of the grid
    ///
    /// This is equal to `grid.width() * grid.height()`
    pub fn size(&self) -> usize {
        self.width * self.height()
    }

    /// Returns an Option to a reference of the cell content at the given coordinate
    ///
    /// # Arguments
    ///
    /// * `coordinate` - The coordinate of the cell
    ///
    pub fn get(&self, coordinate: impl Into<Coordinate>) -> Option<&T> {
        self.items.get(coordinate.into().to_index(self.width))
    }

    /// Returns an Option to a mutable reference of the cell content at the given coordinate
    ///
    /// # Arguments
    ///
    /// * `coordinate` - The coordinate of the cell
    ///
    pub fn get_mut(&mut self, coordinate: impl Into<Coordinate>) -> Option<&mut T> {
        self.items.get_mut(coordinate.into().to_index(self.width))
    }

    /// Yields references to the grid's items
    ///
    /// Iterates over columns, then rows
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    /// Yields mutable references to the grid's items
    ///
    /// Iterates over columns, then rows
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items.iter_mut()
    }

    /// Yields tuples of Coordinate and reference to the grid's items
    pub fn enumerate(&self) -> impl Iterator<Item = (Coordinate, &T)> {
        self.items
            .iter()
            .enumerate()
            .map(|(index, item)| (Coordinate::from_width_and_index(self.width, index), item))
    }

    /// Yields tuples of Coordinate and mutable reference to the grid's items
    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (Coordinate, &mut T)> {
        self.items
            .iter_mut()
            .enumerate()
            .map(|(index, item)| (Coordinate::from_width_and_index(self.width, index), item))
    }

    /// Yields tuples of Coordinate and reference to the grid's items that are neighbors of the given coordinate
    ///
    /// # Arguments
    ///
    /// * `coordinate` - The coordinate who's neighbors shall be yielded
    ///
    pub fn neighbors(
        &self,
        coordinate: impl Into<Coordinate>,
    ) -> impl Iterator<Item = (Coordinate, &T)> {
        let neighbors = self.neighbor_coordinates(coordinate);
        self.enumerate()
            .filter(move |(position, _)| neighbors.iter().any(|neighbor| neighbor == position))
    }

    /// Yields tuples of Coordinate and mutable reference to the grid's items that are neighbors of the given coordinate
    ///
    /// # Arguments
    ///
    /// * `coordinate` - The coordinate whose neighbors shall be yielded
    ///
    pub fn neighbors_mut(
        &mut self,
        coordinate: impl Into<Coordinate>,
    ) -> impl Iterator<Item = (Coordinate, &mut T)> {
        let neighbors = self.neighbor_coordinates(coordinate);
        self.enumerate_mut()
            .filter(move |(position, _)| neighbors.iter().any(|neighbor| neighbor == position))
    }

    /// Yields the rows of the grid
    pub fn rows(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.height()).map(|y| {
            (0..self.width)
                .map(|x| &self.items[Coordinate::new(x, y).to_index(self.width)])
                .collect()
        })
    }

    /// Returns the coordinates that are neighbors of the given coordinate
    ///
    /// # Arguments
    ///
    /// * `coordinate` - The coordinate whose neighbors shall be yielded
    ///
    pub fn neighbor_coordinates(&self, coordinate: impl Into<Coordinate>) -> Vec<Coordinate> {
        coordinate
            .into()
            .neighbors()
            .filter(|coordinate| self.contains(*coordinate))
            .collect()
    }

    /// Determines whether the given coordinate is on the grid
    ///
    /// # Arguments
    ///
    /// * `coordinate` - The coordinate which is to be tested
    ///
    pub fn contains(&self, coordinate: impl Into<Coordinate>) -> bool {
        let coordinate = coordinate.into();
        coordinate.x() < self.width && coordinate.y() < self.height()
    }
}

impl<T> Grid<T>
where
    T: Default,
{
    /// Returns a new instance of Grid for a type that implements the Default trait
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the grid
    /// * `height` - The height of the grid
    ///
    pub fn new_default(width: usize, height: usize) -> Self {
        Self::new(width, height, T::default)
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.rows() {
            if let Some(err) = writeln!(f, "{}", row.iter().join("\t")).err() {
                return Err(err);
            }
        }

        Ok(())
    }
}

impl<T> TryFrom<(T, usize)> for Grid<T::Item>
where
    T: IntoIterator,
{
    type Error = GridConstructionError;

    fn try_from((into_iterator, width): (T, usize)) -> Result<Grid<T::Item>, Self::Error> {
        if width == 0 {
            Err(Self::Error::ZeroSize)
        } else {
            let items = into_iterator.into_iter().collect::<Vec<_>>();

            if items.len() % width != 0 {
                Err(Self::Error::VecSizeNotMultipleOfWidth)
            } else {
                Ok(Grid { width, items })
            }
        }
    }
}
