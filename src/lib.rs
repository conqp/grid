#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    items: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize, initializer: impl Fn() -> T) -> Self {
        Self {
            width,
            height,
            items: (0..height)
                .map(move |_| (0..width).map(|_| initializer()).collect())
                .collect(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width() && y < self.height() {
            Some(&self.items[y][x])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width() && y < self.height() {
            Some(&mut self.items[y][x])
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter().flat_map(|row| row)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.items.iter_mut().flat_map(|row| row)
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.items
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, item)| (x, y, item)))
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.items.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, item)| (x, y, item))
        })
    }

    pub fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize, &T)> {
        self.enumerate().filter(move |(other_x, other_y, _)| {
            is_neighbor(other_x.abs_diff(x), other_y.abs_diff(y))
        })
    }

    pub fn neighbors_mut(
        &mut self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.enumerate_mut().filter(move |(other_x, other_y, _)| {
            is_neighbor(other_x.abs_diff(x), other_y.abs_diff(y))
        })
    }
}

fn is_neighbor(dx: usize, dy: usize) -> bool {
    is_adjunct(dx) && is_adjunct(dy) && !same_field(dx, dy)
}

fn is_adjunct(offset: usize) -> bool {
    offset == 0 || offset == 1
}

fn same_field(dx: usize, dy: usize) -> bool {
    dx == 0 && dy == 0
}
