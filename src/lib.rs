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
        if on_grid(self.width, self.height, x as isize, y as isize) {
            Ok(&self.items[coordinate_to_index(self.width, x, y)])
        } else {
            Err("coordinate not on grid")
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Result<&mut T, &str> {
        if on_grid(self.width, self.height, x as isize, y as isize) {
            let index = coordinate_to_index(self.width, x, y);
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
            let (x, y) = index_to_coordinate(self.width, index);
            (x, y, item)
        })
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.items.iter_mut().enumerate().map(|(index, item)| {
            let (x, y) = index_to_coordinate(self.width, index);
            (x, y, item)
        })
    }

    pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize, &T)> {
        neighbor_coordinates(self.width, self.height, x, y)
            .map(|(x, y)| (x, y, &self.items[coordinate_to_index(self.width, x, y)]))
    }
}

fn coordinate_to_index(width: usize, x: usize, y: usize) -> usize {
    y * width + x
}

fn index_to_coordinate(width: usize, index: usize) -> (usize, usize) {
    let x = index % width;
    let y = (index - x) / width;
    (x, y)
}

fn neighbor_coordinates(
    width: usize,
    height: usize,
    x: usize,
    y: usize,
) -> impl Iterator<Item = (usize, usize)> {
    neighbor_offsets(2)
        .map(move |item| (x as isize + item[0], y as isize + item[1]))
        .filter(move |&(x, y)| on_grid(width, height, x, y))
        .map(|(dx, dy)| (dx as usize, dy as usize))
}

fn on_grid(width: usize, height: usize, x: isize, y: isize) -> bool {
    0 <= x && x < width as isize && 0 <= y && y < height as isize
}

fn neighbor_offsets(dimension: usize) -> impl Iterator<Item = Vec<isize>> {
    (-1..1)
        .map(|index| index as isize)
        .combinations_with_replacement(dimension)
        // skip zero offset
        .filter(|items| !items.iter().all(|&item| item == 0))
}
