use alloc::boxed::Box;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{self, Display, Formatter};
use core::num::NonZero;
use core::ops::{Deref, DerefMut, Index};

use crate::Coordinate;

/// A two-dimensional grid of arbitrary cell content.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Grid<T> {
    width: NonZero<usize>,
    items: Box<[T]>,
}

impl<T> Grid<T> {
    /// Returns a new instance of Grid.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the grid
    /// * `height` - The height of the grid
    /// * `initializer` - A function that takes no arguments and returns an instance of the cell type
    ///
    /// # Panics
    /// This function may panic if the grid size is too lange to fit into a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZero;
    /// use grid2d::Grid;
    ///
    /// let width = NonZero::new(42).unwrap();
    /// let height = NonZero::new(1337).unwrap();
    /// let grid = Grid::new(width, height, String::new);
    ///
    /// assert_eq!(grid.width(), width);
    /// assert_eq!(grid.height(), height);
    /// assert_eq!(grid.size(), width.checked_mul(height).unwrap());
    /// ```
    pub fn new(width: NonZero<usize>, height: NonZero<usize>, initializer: impl Fn() -> T) -> Self {
        Self::try_new(width, height, initializer).expect("grid too large")
    }

    /// Returns a new instance of Grid.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the grid
    /// * `height` - The height of the grid
    /// * `initializer` - A function that takes no arguments and returns an instance of the cell type
    ///
    /// # Errors
    ///
    /// This function returns `None` if the grid size is too lange to fit into a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZero;
    /// use grid2d::Grid;
    ///
    /// let width = NonZero::new(42).unwrap();
    /// let height = NonZero::new(1337).unwrap();
    /// let grid = Grid::try_new(width, height, String::new);
    /// assert!(grid.is_some());
    ///
    /// let width = NonZero::new(usize::MAX).unwrap();
    /// let height = NonZero::new(usize::MAX).unwrap();
    /// let grid = Grid::try_new(width, height, String::new);
    /// assert!(grid.is_none());
    /// ```
    pub fn try_new(
        width: NonZero<usize>,
        height: NonZero<usize>,
        initializer: impl Fn() -> T,
    ) -> Option<Self> {
        let size: usize = width.checked_mul(height)?.get();
        let mut items = Vec::with_capacity(size);
        (0..size).for_each(|_| items.push(initializer()));
        #[allow(unsafe_code)]
        // SAFETY: We perform checked multiplication to ensure that
        // `items.len()` is a multiple of `width`.
        Some(unsafe { Self::new_unchecked(width, items) })
    }

    /// Creates a new grid without checking whether the amount of items is a multiple of width.
    ///
    /// # Safety
    ///
    /// The caller must guarantee that `items.len()` is a non-zero multiple of `width`.
    #[allow(unsafe_code)]
    #[must_use]
    pub unsafe fn new_unchecked(width: NonZero<usize>, items: Vec<T>) -> Self {
        Self {
            width,
            items: items.into_boxed_slice(),
        }
    }

    /// Returns the width of the grid.
    #[must_use]
    pub const fn width(&self) -> NonZero<usize> {
        self.width
    }

    /// Returns the height of the grid.
    #[must_use]
    pub fn height(&self) -> NonZero<usize> {
        #[allow(unsafe_code)]
        // SAFETY: Both `width` and `height` are always a non-zero multiple of the Grid's size.
        unsafe {
            NonZero::new_unchecked(self.size().get() / self.width)
        }
    }

    /// Returns the size of the grid.
    ///
    /// This is equal to `grid.width() * grid.height()`
    #[must_use]
    pub const fn size(&self) -> NonZero<usize> {
        #[allow(unsafe_code)]
        // SAFETY: We never allow to construct a `Grid` with `width` or height `zero`.
        // Additionally, we perform checked multiplication when constructing a `Grid`.
        // Thus, a Grid can never have a size of zero.
        unsafe {
            NonZero::new_unchecked(self.items.len())
        }
    }

    /// Returns true, if the grid is empty, else false.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns an Option to a reference of the cell content at the given coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZero;
    /// use grid2d::Grid;
    ///
    /// let grid = Grid::try_from(("Hello world!".chars(), NonZero::new(4).unwrap())).unwrap();
    /// assert_eq!(grid.get((0, 0)).unwrap(), &'H');
    /// assert_eq!(grid.get((1, 0)).unwrap(), &'e');
    /// assert_eq!(grid.get((2, 0)).unwrap(), &'l');
    /// assert_eq!(grid.get((3, 0)).unwrap(), &'l');
    /// assert_eq!(grid.get((4, 0)), None);
    /// assert_eq!(grid.get((0, 1)).unwrap(), &'o');
    /// assert_eq!(grid.get((1, 1)).unwrap(), &' ');
    /// assert_eq!(grid.get((2, 1)).unwrap(), &'w');
    /// assert_eq!(grid.get((3, 1)).unwrap(), &'o');
    /// assert_eq!(grid.get((4, 1)), None);
    /// assert_eq!(grid.get((0, 2)).unwrap(), &'r');
    /// assert_eq!(grid.get((1, 2)).unwrap(), &'l');
    /// assert_eq!(grid.get((2, 2)).unwrap(), &'d');
    /// assert_eq!(grid.get((3, 2)).unwrap(), &'!');
    /// assert_eq!(grid.get((4, 2)), None);
    /// assert_eq!(grid.get((0, 3)), None);
    /// ```
    #[inline]
    pub fn get(&self, coordinate: impl Into<Coordinate>) -> Option<&T> {
        coordinate
            .into()
            .as_index(self.width)
            .and_then(|index| self.items.get(index))
    }

    /// Returns an Option to a mutable reference of the cell content at the given coordinate.
    #[inline]
    pub fn get_mut(&mut self, coordinate: impl Into<Coordinate>) -> Option<&mut T> {
        coordinate
            .into()
            .as_index(self.width)
            .and_then(|index| self.items.get_mut(index))
    }

    /// Yields references to the grid's items.
    ///
    /// Iterates over columns, then rows.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    /// Yields mutable references to the grid's items.
    ///
    /// Iterates over columns, then rows.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZero;
    /// use grid2d::{Grid, Coordinate};
    ///
    /// let width = NonZero::new(3).unwrap();
    /// let height = NonZero::new(4).unwrap();
    /// let mut grid = Grid::new(width, height, String::new);
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

    /// Yields tuples of Coordinate and reference to the grid's items.
    pub fn enumerate(&self) -> impl Iterator<Item = (Coordinate, &T)> {
        self.items
            .iter()
            .enumerate()
            .map(|(index, item)| (Coordinate::from_width_and_index(self.width, index), item))
    }

    /// Yields tuples of Coordinate and mutable reference to the grid's items.
    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (Coordinate, &mut T)> {
        self.items
            .iter_mut()
            .enumerate()
            .map(|(index, item)| (Coordinate::from_width_and_index(self.width, index), item))
    }

    /// Yields tuples of Coordinate and reference to the grid's items that are neighbors of the given coordinate.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZero;
    /// use grid2d::{Grid, Coordinate};
    ///
    /// let width = NonZero::new(3).unwrap();
    /// let height = NonZero::new(4).unwrap();
    /// let mut grid = Grid::new(width, height, String::new);
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
        self.neighbors_internal(self.neighbor_coordinates(coordinate))
    }

    #[inline]
    fn neighbors_internal(
        &self,
        neighbors: Vec<Coordinate>,
    ) -> impl Iterator<Item = (Coordinate, &T)> {
        self.enumerate()
            .filter(move |(position, _)| neighbors.iter().any(|neighbor| neighbor == position))
    }

    /// Yields tuples of Coordinate and mutable reference to the grid's items that are neighbors of the given coordinate.
    pub fn neighbors_mut(
        &mut self,
        coordinate: impl Into<Coordinate>,
    ) -> impl Iterator<Item = (Coordinate, &mut T)> {
        self.neighbors_mut_internal(self.neighbor_coordinates(coordinate))
    }

    #[inline]
    fn neighbors_mut_internal(
        &mut self,
        neighbors: Vec<Coordinate>,
    ) -> impl Iterator<Item = (Coordinate, &mut T)> {
        self.enumerate_mut()
            .filter(move |(position, _)| neighbors.iter().any(|neighbor| neighbor == position))
    }

    /// Yields the rows of the grid.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZero;
    /// use grid2d::Grid;
    ///
    /// let grid = Grid::try_from((0u8..6, NonZero::<usize>::new(2).unwrap())).unwrap();
    /// let rows: [[u8; 2]; 3] = [[0, 1], [2, 3], [4, 5]];
    ///
    /// for (row, target) in grid.rows().zip(rows.iter()) {
    ///     for (column, target) in row.zip(target) {
    ///         assert_eq!(column, target);
    ///     }
    /// }
    /// ```
    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.height().get()).map(move |y| {
            (0..self.width.get()).filter_map(move |x| {
                Coordinate::new(x, y)
                    .as_index(self.width)
                    .map(|index| &self.items[index])
            })
        })
    }

    /// Returns the coordinates that are neighbors of the given coordinate.
    pub fn neighbor_coordinates(&self, coordinate: impl Into<Coordinate>) -> Vec<Coordinate> {
        coordinate
            .into()
            .neighbors()
            .filter(|coordinate| self.encompasses(*coordinate))
            .collect()
    }

    /// Determines whether the given coordinate is on the grid.
    pub fn encompasses(&self, coordinate: impl Into<Coordinate>) -> bool {
        self.encompasses_internal(coordinate.into())
    }

    #[inline]
    fn encompasses_internal(&self, coordinate: Coordinate) -> bool {
        coordinate.x() < self.width.get() && coordinate.y() < self.height().get()
    }
}

impl<T> Grid<T>
where
    T: Default,
{
    /// Returns a new instance of Grid for a type that implements the Default trait.
    pub fn new_default(width: NonZero<usize>, height: NonZero<usize>) -> Self {
        Self::new(width, height, T::default)
    }
}

impl<T> Grid<T>
where
    T: PartialEq,
{
    /// Determines whether the grid contains the given element.
    pub fn contains(&self, element: &T) -> bool {
        self.items.contains(element)
    }
}

impl<T> AsMut<[T]> for Grid<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.items
    }
}

impl<T> AsRef<[T]> for Grid<T> {
    fn as_ref(&self) -> &[T] {
        &self.items
    }
}

impl<T> Borrow<[T]> for Grid<T> {
    fn borrow(&self) -> &[T] {
        &self.items
    }
}

impl<T> Deref for Grid<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

/// Display the grid.
///
/// # Examples
///
/// ```
/// use std::num::NonZero;
/// use grid2d::Grid;
///
/// const REFERENCE: &str = "0\t1\n2\t3\n4\t5";
///
/// let grid = Grid::try_from((0u8..6, NonZero::<usize>::new(2).unwrap())).unwrap();
/// let string = grid.to_string();
///
/// assert_eq!(string, REFERENCE);
/// ```
impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let max_x = self.width().get().saturating_sub(1);
        let max_y = self.height().get().saturating_sub(1);

        for (y, row) in self.rows().enumerate() {
            for (x, element) in row.enumerate() {
                write!(f, "{element}")?;

                if x < max_x {
                    write!(f, "\t")?;
                }
            }

            if y < max_y {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

/// Return the element at the respective position.
///
/// # Examples
///
/// ```
/// use std::num::NonZero;
/// use grid2d::Grid;
///
/// let grid = Grid::try_from((0u8..6, NonZero::<usize>::new(2).unwrap())).unwrap();
///
/// assert_eq!(grid[(0, 0)], 0);
/// assert_eq!(grid[(1, 0)], 1);
/// assert_eq!(grid[(0, 1)], 2);
/// assert_eq!(grid[(1, 1)], 3);
/// assert_eq!(grid[(0, 2)], 4);
/// assert_eq!(grid[(1, 2)], 5);
/// ```
impl<T, I> Index<I> for Grid<T>
where
    I: Into<Coordinate>,
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

/// Create a Grid from a tuple of an iterable and the desired width.
///
/// # Examples
///
/// ```
/// use std::num::NonZero;
/// use grid2d::Grid;
///
/// let items = vec![1, 2, 3, 4, 5, 6, 7, 8];
/// let width = NonZero::new(4).unwrap();
/// let items2 = [1, 2, 3, 4, 5, 6, 7, 8];
/// let width2 = NonZero::new(3).unwrap();
///
/// assert!(Grid::try_from((items.clone(), width)).is_ok());
/// assert!(Grid::try_from((items.clone().iter(), width)).is_ok());
/// assert!(Grid::try_from((items2, width)).is_ok());
/// assert_eq!(
///     Grid::try_from((items.clone(), width2)),
///     Err(())
/// );
/// ```
impl<T> TryFrom<(T, NonZero<usize>)> for Grid<T::Item>
where
    T: IntoIterator,
{
    type Error = ();

    fn try_from((into_iterator, width): (T, NonZero<usize>)) -> Result<Self, Self::Error> {
        let items = into_iterator.into_iter().collect::<Vec<_>>();

        if items.len() % width == 0 {
            #[allow(unsafe_code)]
            // SAFETY: In the line above, we checked that `items.len()` is a multiple of `width`.
            Ok(unsafe { Self::new_unchecked(width, items) })
        } else {
            Err(())
        }
    }
}
