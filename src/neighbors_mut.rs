use alloc::vec::IntoIter;
use core::mem::take;
use core::num::NonZero;

use crate::Coordinate;

pub struct NeighborsMut<'a, T> {
    width: NonZero<usize>,
    items: &'a mut [T],
    next_index: usize,
    coordinates: IntoIter<Coordinate>,
}

impl<'a, T> NeighborsMut<'a, T> {
    pub(crate) const fn new(
        width: NonZero<usize>,
        items: &'a mut [T],
        coordinates: IntoIter<Coordinate>,
    ) -> Self {
        Self {
            width,
            items,
            next_index: 0,
            coordinates,
        }
    }
}

impl<'a, T> Iterator for NeighborsMut<'a, T> {
    type Item = (Coordinate, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let coordinate = self.coordinates.next()?;
        let index = coordinate.as_index(self.width)?;
        let offset = index.checked_sub(self.next_index)?;
        let items = take(&mut self.items);

        if offset >= items.len() {
            self.items = items;
            return None;
        }

        let (_, items) = items.split_at_mut(offset);
        let (item, items) = items.split_first_mut()?;
        self.items = items;
        self.next_index = index + 1;
        Some((coordinate, item))
    }
}
