use crate::coordinate::Coordinate;
use crate::errors::GridConstructionError;
use itertools::Itertools;
use std::fmt::{Display, Formatter};

/// A two-dimensional grid of arbitrary cell content
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid<T> {
    width: usize,
    items: Box<[T]>,
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
    /// # Examples
    ///
    /// ```
    /// use grid2d::Grid;
    ///
    /// let width = 42;
    /// let height = 1337;
    /// let grid = Grid::new(width, height, String::new);
    ///
    /// assert_eq!(grid.width(), width);
    /// assert_eq!(grid.height(), height);
    /// assert_eq!(grid.size(), width * height);
    ///
    /// assert_eq!(Grid::new(2, 0, String::new).height(), 0);
    /// assert_eq!(Grid::new(0, 3, String::new).width(), 0);
    /// ```
    pub fn new(width: usize, height: usize, initializer: impl Fn() -> T) -> Self {
        Self::init(width, (0..width * height).map(|_| initializer()).collect())
    }

    fn init(width: usize, items: Vec<T>) -> Self {
        Self {
            width,
            items: items.into_boxed_slice(),
        }
    }

    /// Returns the width of the grid
    #[must_use]
    pub const fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid
    #[must_use]
    pub const fn height(&self) -> usize {
        if self.width == 0 {
            0
        } else {
            self.size() / self.width
        }
    }

    /// Returns the size of the grid
    ///
    /// This is equal to `grid.width() * grid.height()`
    #[must_use]
    pub const fn size(&self) -> usize {
        self.items.len()
    }

    /// Returns true, if the grid is empty, else false
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns an Option to a reference of the cell content at the given coordinate
    ///
    /// # Arguments
    ///
    /// * `coordinate` - The coordinate of the cell
    ///
    /// # Examples
    ///
    /// ```
    /// use grid2d::Grid;
    ///
    /// let grid = Grid::try_from(("Hello world!".chars(), 4)).unwrap();
    /// assert_eq!(grid.get((0, 2)).unwrap(), &'r');
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// use grid2d::{Grid, Coordinate};
    /// let mut grid = Grid::new(3, 4, String::new);
    /// let text = "Hello world!";
    ///
    /// for (index, item) in grid.iter_mut().enumerate() {
    ///     item.push(text.as_bytes()[index] as char);
    /// }
    ///
    /// assert_eq!(grid.get(Coordinate::new(0, 0)).unwrap(), "H");
    /// assert_eq!(grid.get(Coordinate::new(1, 0)).unwrap(), "e");
    /// assert_eq!(grid.get(Coordinate::new(2, 0)).unwrap(), "l");
    /// assert_eq!(grid.get(Coordinate::new(0, 1)).unwrap(), "l");
    /// assert_eq!(grid.get(Coordinate::new(1, 1)).unwrap(), "o");
    /// assert_eq!(grid.get(Coordinate::new(2, 1)).unwrap(), " ");
    /// assert_eq!(grid.get(Coordinate::new(0, 2)).unwrap(), "w");
    /// assert_eq!(grid.get(Coordinate::new(1, 2)).unwrap(), "o");
    /// assert_eq!(grid.get(Coordinate::new(2, 2)).unwrap(), "r");
    /// assert_eq!(grid.get(Coordinate::new(0, 3)).unwrap(), "l");
    /// assert_eq!(grid.get(Coordinate::new(1, 3)).unwrap(), "d");
    /// assert_eq!(grid.get(Coordinate::new(2, 3)).unwrap(), "!");
    /// ```
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
    /// # Examples
    ///
    /// ```
    /// use grid2d::{Grid, Coordinate};
    ///
    /// let mut grid = Grid::new(3, 4, String::new);
    /// let text = "Hello world!";
    /// let neighbors: [[&str; 3]; 4] = [
    ///     ["H", "e", "l"],
    ///     ["l", "o", " "],
    ///     ["w", "o", "r"],
    ///     ["l", "d", "!"],
    /// ];
    ///
    /// for (index, item) in grid.iter_mut().enumerate() {
    ///     item.push(text.chars().nth(index).unwrap());
    /// }
    ///
    /// for (coordinate, neighbor) in grid.neighbors(Coordinate::new(1, 1)) {
    ///     assert_eq!(neighbor, neighbors[coordinate.y()][coordinate.x()]);
    /// }
    ///
    /// assert_eq!(grid.neighbors(Coordinate::new(0, 0)).count(), 3);
    /// assert_eq!(grid.neighbors(Coordinate::new(0, 1)).count(), 5);
    /// assert_eq!(grid.neighbors(Coordinate::new(0, 2)).count(), 5);
    /// assert_eq!(grid.neighbors(Coordinate::new(0, 3)).count(), 3);
    /// assert_eq!(grid.neighbors(Coordinate::new(1, 0)).count(), 5);
    /// assert_eq!(grid.neighbors(Coordinate::new(1, 1)).count(), 8);
    /// assert_eq!(grid.neighbors(Coordinate::new(1, 2)).count(), 8);
    /// assert_eq!(grid.neighbors(Coordinate::new(1, 3)).count(), 5);
    /// assert_eq!(grid.neighbors(Coordinate::new(2, 0)).count(), 3);
    /// assert_eq!(grid.neighbors(Coordinate::new(2, 1)).count(), 5);
    /// assert_eq!(grid.neighbors(Coordinate::new(2, 2)).count(), 5);
    /// assert_eq!(grid.neighbors(Coordinate::new(2, 3)).count(), 3);
    /// ```
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
            .filter(|coordinate| self.encompasses(*coordinate))
            .collect()
    }

    /// Determines whether the given coordinate is on the grid
    ///
    /// # Arguments
    ///
    /// * `coordinate` - The coordinate which is to be tested
    ///
    pub fn encompasses(&self, coordinate: impl Into<Coordinate>) -> bool {
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

impl<T> Grid<T>
where
    T: PartialEq,
{
    /// Determines whether the grid contains the given element
    ///
    /// # Arguments
    ///
    /// * `item` - The coordinate which is to be tested
    ///
    pub fn contains(&self, element: &T) -> bool {
        self.items.contains(element)
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.rows() {
            writeln!(f, "{}", row.iter().join("\t"))?;
        }

        Ok(())
    }
}

/// Create a Grid from a tuple of an iterable and the desired width.
///
/// # Examples
///
/// ```
/// use grid2d::{Grid, GridConstructionError};
///
/// let items = vec![1, 2, 3, 4, 5, 6, 7, 8];
/// let items2 = [1, 2, 3, 4, 5, 6, 7, 8];
///
/// assert!(Grid::try_from((items.clone(), 4)).is_ok());
/// assert!(Grid::try_from((items.clone().iter(), 4)).is_ok());
/// assert!(Grid::try_from((items2, 4)).is_ok());
/// assert_eq!( Grid::try_from((items.clone(), 3)).err(), Some(GridConstructionError::VecSizeNotMultipleOfWidth));
/// assert_eq!(Grid::try_from((items.clone(), 0)).err(), Some(GridConstructionError::ZeroSize));
/// ```
impl<T> TryFrom<(T, usize)> for Grid<T::Item>
where
    T: IntoIterator,
{
    type Error = GridConstructionError;

    fn try_from((into_iterator, width): (T, usize)) -> Result<Self, Self::Error> {
        match width {
            0 => Err(GridConstructionError::ZeroSize),
            width => {
                let items = into_iterator.into_iter().collect::<Vec<_>>();

                if items.len() % width == 0 {
                    Ok(Self::init(width, items))
                } else {
                    Err(GridConstructionError::VecSizeNotMultipleOfWidth)
                }
            }
        }
    }
}
