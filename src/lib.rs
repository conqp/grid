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
            items: (0..height * width).map(|_| initializer()).collect(),
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

    pub fn get(&self, x: usize, y: usize) -> Result<&T, &str> {
        if self.on_grid(x as isize, y as isize) {
            Ok(&self.items[self.coordinate_to_index(x, y)])
        } else {
            Err("coordinate not on grid")
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut T, &str> {
        if self.on_grid(x as isize, y as isize) {
            let index = self.coordinate_to_index(x, y);
            Ok(&mut self.items[index])
        } else {
            Err("coordinate not on grid")
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
            let (x, y) = self.index_to_coordinate(index);
            (x, y, item)
        })
    }

    pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize, &T)> {
        self.neighbor_indices(x, y)
            .map(|(x, y)| (x, y, &self.items[self.coordinate_to_index(x, y)]))
    }

    fn coordinate_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn index_to_coordinate(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = (index - x) / self.width;
        (x, y)
    }

    fn neighbor_indices(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        neighbor_offsets(2)
            .map(move |item| (x as isize + item[0], y as isize + item[1]))
            .filter(|&(x, y)| self.on_grid(x, y))
            .map(|(dx, dy)| (dx as usize, dy as usize))
    }

    fn on_grid(&self, x: isize, y: isize) -> bool {
        0 <= x && x < self.width as isize && 0 <= y && y < self.height as isize
    }
}

fn neighbor_offsets(dimension: usize) -> impl Iterator<Item = Vec<isize>> {
    (-1..1)
        .map(|index| index as isize)
        .combinations_with_replacement(dimension)
        // skip zero offset
        .filter(|items| !items.iter().all(|&item| item == 0))
}
