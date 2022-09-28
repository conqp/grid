use itertools::Itertools;

#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    items: Vec<T>,
}

const NEIGHBOR_OFFSETS: [isize; 3] = [-1, 0, 1];

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, initializer: impl Fn() -> T) -> Self {
        Self {
            width,
            items: (0..height * width).map(|_| initializer()).collect(),
        }
    }

    pub fn from_vec(vec: Vec<T>, width: usize) -> Result<Self, &'static str> {
        if vec.len() % width == 0 {
            Ok(Self {
                width: width,
                items: vec,
            })
        } else {
            Err("iterator size must be a multiple of width")
        }
    }

    pub fn from_iter(
        iterator: impl Iterator<Item = T>,
        width: usize,
    ) -> Result<Self, &'static str> {
        Self::from_vec(iterator.collect(), width)
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
            let x = index % self.width;
            let y = (index - x) / self.width;
            (x, y, item)
        })
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.items.iter_mut().enumerate().map(|(index, item)| {
            let x = index % self.width;
            let y = (index - x) / self.width;
            (x, y, item)
        })
    }

    pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize, &T)> {
        self.enumerate().filter(move |&(ix, iy, _)| {
            self.neighbor_coordinates(x, y)
                .any(|(nx, ny)| nx == ix && ny == iy)
        })
    }

    pub fn neighbors_mut(
        &mut self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize, &mut T)> {
        let neigbors = self.neighbor_coordinates(x, y).collect_vec();
        self.enumerate_mut()
            .filter(move |&(x, y, _)| neigbors.iter().any(|&(nx, ny)| nx == x && ny == y))
    }

    pub fn rows(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.height()).map(|y| {
            (0..self.width)
                .map(|x| &self.items[self.coordinate_to_index(x, y)])
                .collect::<Vec<&T>>()
        })
    }

    fn neighbor_coordinates(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        neighbor_offsets()
            .map(move |(dx, dy)| (x as isize + dx, y as isize + dy))
            .filter(move |&(x, y)| self.on_grid(x, y))
            .map(|(dx, dy)| (dx as usize, dy as usize))
    }

    fn on_grid(&self, x: isize, y: isize) -> bool {
        0 <= x && x < self.width as isize && 0 <= y && y < self.height() as isize
    }

    fn coordinate_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

fn neighbor_offsets() -> impl Iterator<Item = (isize, isize)> {
    NEIGHBOR_OFFSETS
        .into_iter()
        .cartesian_product(NEIGHBOR_OFFSETS)
        // skip zero offset
        .filter(|&(x, y)| !(x == 0 && y == 0))
}
