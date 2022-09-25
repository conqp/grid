#[derive(Debug)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    items: Vec<T>,
}

const NEIGHBOR_OFFSETS: [isize; 3] = [-1, 0, 1];

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
        self.items.iter().enumerate().map(|(idx, item)| {
            let (x, y) = self.index_to_coordinate(idx);
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

    fn index_to_coordinate(&self, index: usize) -> (usize, usize) {
        let x = index % self.width;
        let y = (index - x) / self.width;
        (x, y)
    }
}

fn neighbor_offsets() -> impl Iterator<Item = (isize, isize)> {
    NEIGHBOR_OFFSETS
        .into_iter()
        .flat_map(|i| [i].into_iter().cycle().zip(NEIGHBOR_OFFSETS))
        .filter(|(x, y)| !(*x == 0 && *y == 0))
}
