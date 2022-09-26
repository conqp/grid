use itertools::Itertools;

#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    items: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, initializer: impl Fn() -> T) -> Self {
        Self {
            width,
            height,
            items: (0..height)
                .flat_map(|_| (0..width).map(|_| initializer()))
                .collect(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width() && y < self.height() {
            Some(&self.items[self.coordinate_to_index(x, y)])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width() && y < self.height() {
            let index = self.coordinate_to_index(x, y);
            Some(&mut self.items[index])
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items.iter_mut()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.items.iter().enumerate().map(|(index, item)| {
            let (x, y) = index_to_coordinate(index, self.width);
            (x, y, item)
        })
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        let width = self.width;
        self.items.iter_mut().enumerate().map(move |(index, item)| {
            let (x, y) = index_to_coordinate(index, width);
            (x, y, item)
        })
    }

    pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize, &T)> {
        self.neighbor_indices(x, y)
            .map(|(x, y)| (x, y, self.get(x, y).unwrap()))
    }

    fn neighbor_indices(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        neighbor_offsets()
            .map(move |(dx, dy)| (x as isize + dx, y as isize + dy))
            .filter(|(dx, dy)| {
                // remove elements outside of grid
                0 <= *dx && *dx < self.width as isize && 0 <= *dy && *dy < self.height as isize
            })
            .map(|(dx, dy)| (dx as usize, dy as usize))
    }

    fn coordinate_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

fn neighbor_offsets() -> impl Iterator<Item = (isize, isize)> {
    (-1..1).combinations_with_replacement(2).map(|items| (items[0] as isize, items[1] as isize))
}

fn index_to_coordinate(index: usize, width: usize) -> (usize, usize) {
    let x = index % width;
    let y = (index - x) / width;
    (x, y)
}
