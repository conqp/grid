use itertools::Itertools;

mod coordinate;
pub use coordinate::Coordinate;
pub use coordinate::CoordinateParseError;

#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    items: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, initializer: impl Fn() -> T) -> Self {
        Self {
            width,
            items: (0..width * height).map(|_| initializer()).collect_vec(),
        }
    }

    pub fn from_vec(vec: Vec<T>, width: usize) -> Result<Self, &'static str> {
        if vec.len() % width == 0 {
            Ok(Self { width, items: vec })
        } else {
            Err("vec size must be a multiple of width")
        }
    }

    pub fn from_iter(
        iterator: impl Iterator<Item = T>,
        width: usize,
    ) -> Result<Self, &'static str> {
        Self::from_vec(iterator.collect_vec(), width)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.items.len() / self.width
    }

    pub fn size(&self) -> usize {
        self.width * self.height()
    }

    pub fn get(&self, coordinate: impl Into<Coordinate>) -> Option<&T> {
        self.items.get(coordinate.into().to_index(self.width))
    }

    pub fn get_mut(&mut self, coordinate: impl Into<Coordinate>) -> Option<&mut T> {
        self.items.get_mut(coordinate.into().to_index(self.width))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items.iter_mut()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Coordinate, &T)> {
        self.items
            .iter()
            .enumerate()
            .map(|(index, item)| (Coordinate::from_width_and_index(self.width, index), item))
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (Coordinate, &mut T)> {
        self.items
            .iter_mut()
            .enumerate()
            .map(|(index, item)| (Coordinate::from_width_and_index(self.width, index), item))
    }

    pub fn neighbors(
        &self,
        coordinate: impl Into<Coordinate>,
    ) -> impl Iterator<Item = (Coordinate, &T)> {
        let neighbors = self.neighbor_coordinates(coordinate);
        self.enumerate()
            .filter(move |(position, _)| neighbors.iter().any(|neighbor| neighbor == position))
    }

    pub fn neighbors_mut(
        &mut self,
        coordinate: impl Into<Coordinate>,
    ) -> impl Iterator<Item = (Coordinate, &mut T)> {
        let neighbors = self.neighbor_coordinates(coordinate);
        self.enumerate_mut()
            .filter(move |(position, _)| neighbors.iter().any(|neighbor| neighbor == position))
    }

    pub fn rows(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.height()).map(|y| {
            (0..self.width)
                .map(|x| &self.items[Coordinate::new(x, y).to_index(self.width)])
                .collect_vec()
        })
    }

    pub fn neighbor_coordinates(&self, coordinate: impl Into<Coordinate>) -> Vec<Coordinate> {
        coordinate
            .into()
            .neighbors()
            .into_iter()
            .filter(|coordinate| self.contains(*coordinate))
            .collect_vec()
    }

    pub fn contains(&self, coordinate: impl Into<Coordinate>) -> bool {
        let coordinate = coordinate.into();
        coordinate.x() < self.width && coordinate.y() < self.height()
    }
}
