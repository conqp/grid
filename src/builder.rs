use alloc::vec::Vec;
use core::num::NonZero;
use core::ops::{Deref, DerefMut};

use crate::{BuildError, Grid};

/// A builder to construct a `Grid`.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct GridBuilder<T> {
    items: Vec<T>,
    width: Option<NonZero<usize>>,
    height: Option<NonZero<usize>>,
}

impl<T> GridBuilder<T> {
    /// Create a new `GridBuilder` with the respective items.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZero;
    /// use grid2d::{Grid, GridBuilder};
    ///
    /// let items = vec![1, 2, 3, 4, 5, 6];
    /// let width = NonZero::new(2).unwrap();
    /// let result = GridBuilder::new(items).width(width).build();
    ///
    /// assert_eq!(result.ok(), Grid::try_from((vec![1, 2, 3, 4, 5, 6], width)).ok());
    /// ```
    #[must_use]
    pub const fn new(items: Vec<T>) -> Self {
        Self {
            items,
            width: None,
            height: None,
        }
    }

    /// Set the desired width.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZero;
    /// use grid2d::{Grid, GridBuilder};
    ///
    /// let items = vec![1, 2, 3, 4, 5, 6];
    /// let width = NonZero::new(2).unwrap();
    /// let result = GridBuilder::new(items).width(width).build();
    ///
    /// assert_eq!(result.ok(), Grid::try_from((vec![1, 2, 3, 4, 5, 6], width)).ok());
    /// ```
    #[must_use]
    pub const fn width(mut self, width: NonZero<usize>) -> Self {
        self.width.replace(width);
        self
    }

    /// Set the desired height.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZero;
    /// use grid2d::{Grid, GridBuilder};
    ///
    /// let items = vec![1, 2, 3, 4, 5, 6];
    /// let height = NonZero::new(2).unwrap();
    /// let width = NonZero::new(items.len() / height).unwrap();
    /// let result = GridBuilder::new(items).height(height).build();
    ///
    /// assert_eq!(result.ok(), Grid::try_from((vec![1, 2, 3, 4, 5, 6], width)).ok());
    /// ```
    #[must_use]
    pub const fn height(mut self, height: NonZero<usize>) -> Self {
        self.height.replace(height);
        self
    }

    /// Add the respective item to the builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZero;
    /// use grid2d::{Grid, GridBuilder};
    ///
    /// let items = vec![1, 2, 3, 4, 5];
    /// let item = 6;
    /// let width = NonZero::new(2).unwrap();
    /// let result = GridBuilder::new(items).width(width).with_item(item).build();
    ///
    /// assert_eq!(result.ok(), Grid::try_from((vec![1, 2, 3, 4, 5, 6], width)).ok());
    /// ```
    #[must_use]
    pub fn with_item(mut self, item: T) -> Self {
        self.items.push(item);
        self
    }

    /// Extend the items with the given iterable.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZero;
    /// use grid2d::{Grid, GridBuilder};
    ///
    /// let items = vec![1, 2, 3];
    /// let more_items = vec![4, 5, 6];
    /// let width = NonZero::new(2).unwrap();
    /// let result = GridBuilder::new(items).width(width).extend(more_items).build();
    ///
    /// assert_eq!(result.ok(), Grid::try_from((vec![1, 2, 3, 4, 5, 6], width)).ok());
    /// ```
    #[must_use]
    pub fn extend<I>(mut self, iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        self.items.extend(iter);
        self
    }

    /// Build the grid.
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] in case the build fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZero;
    /// use grid2d::{BuildError, Grid, GridBuilder};
    ///
    /// // OK
    /// let items = vec![1, 2, 3, 4, 5, 6];
    /// let width = NonZero::new(2).unwrap();
    /// let result = GridBuilder::new(items).width(width).build();
    /// assert_eq!(result.ok(), Grid::try_from((vec![1, 2, 3, 4, 5, 6], width)).ok());
    ///
    /// // Neither width nor height set
    /// let items = vec![1, 2, 3, 4, 5];
    /// let result = GridBuilder::new(items).build();
    /// assert_eq!(result, Err(BuildError::NeitherWidthNotHeightSet(vec![1, 2, 3, 4, 5])));
    ///
    /// // Width too large
    /// let items = vec![1, 2, 3, 4, 5];
    /// let width = NonZero::new(6).unwrap();
    /// let result = GridBuilder::new(items).width(width).build();
    /// assert_eq!(result, Err(BuildError::TooWide(vec![1, 2, 3, 4, 5])));
    ///
    /// // Height too large
    /// let items = vec![1, 2, 3, 4, 5];
    /// let height = NonZero::new(6).unwrap();
    /// let result = GridBuilder::new(items).height(height).build();
    /// assert_eq!(result, Err(BuildError::TooTall(vec![1, 2, 3, 4, 5])));
    ///
    /// // Size is not a multiple of the width
    /// let items = vec![1, 2, 3, 4, 5, 6];
    /// let width = NonZero::new(3).unwrap();
    /// let height = NonZero::new(3).unwrap();
    /// let result = GridBuilder::new(items).width(width).height(height).build();
    /// assert_eq!(result, Err(BuildError::SizeDoesNotMatch(vec![1, 2, 3, 4, 5, 6])));
    ///
    /// // Size is not a multiple of the width
    /// let items = vec![1, 2, 3, 4, 5];
    /// let width = NonZero::new(3).unwrap();
    /// let result = GridBuilder::new(items).width(width).build();
    /// assert_eq!(result, Err(BuildError::SizeNotMultipleOfWidth(vec![1, 2, 3, 4, 5])));
    ///
    /// // Size is not a multiple of the height
    /// let items = vec![1, 2, 3, 4, 5];
    /// let height = NonZero::new(2).unwrap();
    /// let result = GridBuilder::new(items).height(height).build();
    /// assert_eq!(result, Err(BuildError::SizeNotMultipleOfHeight(vec![1, 2, 3, 4, 5])));
    /// ```
    pub fn build(self) -> Result<Grid<T>, BuildError<T>> {
        match (self.width, self.height) {
            (Some(width), Some(height)) => {
                if width
                    .checked_mul(height)
                    .is_some_and(|size| self.items.len() == size.get())
                {
                    #[allow(unsafe_code)]
                    // SAFETY: We just checked that the width and height match the items size.
                    Ok(unsafe { Grid::new_unchecked(width, self.items) })
                } else {
                    Err(BuildError::SizeDoesNotMatch(self.items))
                }
            }
            (Some(width), None) => {
                if width.get() > self.items.len() {
                    Err(BuildError::TooWide(self.items))
                } else if self.items.len() % width != 0 {
                    Err(BuildError::SizeNotMultipleOfWidth(self.items))
                } else {
                    #[allow(unsafe_code)]
                    // SAFETY: We just checked that the width matches the items size.
                    Ok(unsafe { Grid::new_unchecked(width, self.items) })
                }
            }
            (None, Some(height)) => {
                if height.get() > self.items.len() {
                    Err(BuildError::TooTall(self.items))
                } else if let Some(width) =
                    NonZero::new(self.items.len() / height.get()).and_then(|width| {
                        if self.items.len() % width == 0 {
                            Some(width)
                        } else {
                            None
                        }
                    })
                {
                    #[allow(unsafe_code)]
                    // SAFETY: We just checked that the width matches the items size.
                    Ok(unsafe { Grid::new_unchecked(width, self.items) })
                } else {
                    Err(BuildError::SizeNotMultipleOfHeight(self.items))
                }
            }
            (None, None) => Err(BuildError::NeitherWidthNotHeightSet(self.items)),
        }
    }
}

impl<T> AsRef<Vec<T>> for GridBuilder<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.items
    }
}

impl<T> AsMut<Vec<T>> for GridBuilder<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.items
    }
}

impl<T> Deref for GridBuilder<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<T> DerefMut for GridBuilder<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl<T> From<Vec<T>> for GridBuilder<T> {
    fn from(items: Vec<T>) -> Self {
        Self::new(items)
    }
}
