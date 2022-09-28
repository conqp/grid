use grid::Grid;

#[test]
fn zero_width() {
    let grid = Grid::new(0, 3, String::new);
    assert_eq!(grid.width(), 0);
}

#[test]
fn zero_height() {
    let grid = Grid::new(2, 0, String::new);
    assert_eq!(grid.height(), 0);
}

#[test]
fn size() {
    let width = 42;
    let height = 1337;
    let grid = Grid::new(width, height, String::new);
    assert_eq!(grid.width(), width);
    assert_eq!(grid.height(), height);
    assert_eq!(grid.size(), width * height);
}

#[test]
fn storage() {
    let mut grid = Grid::new(3, 4, String::new);
    let text = "Hello world!";

    for (index, item) in grid.iter_mut().enumerate() {
        item.push(text.as_bytes()[index] as char);
    }

    assert_eq!(grid.get(0, 0).unwrap(), "H");
    assert_eq!(grid.get(1, 0).unwrap(), "e");
    assert_eq!(grid.get(2, 0).unwrap(), "l");
    assert_eq!(grid.get(0, 1).unwrap(), "l");
    assert_eq!(grid.get(1, 1).unwrap(), "o");
    assert_eq!(grid.get(2, 1).unwrap(), " ");
    assert_eq!(grid.get(0, 2).unwrap(), "w");
    assert_eq!(grid.get(1, 2).unwrap(), "o");
    assert_eq!(grid.get(2, 2).unwrap(), "r");
    assert_eq!(grid.get(0, 3).unwrap(), "l");
    assert_eq!(grid.get(1, 3).unwrap(), "d");
    assert_eq!(grid.get(2, 3).unwrap(), "!");
}

#[test]
fn neighbors() {
    let mut grid = Grid::new(3, 4, String::new);
    let text = "Hello world!";
    let neighbors: [[&str; 3]; 4] = [
        ["H", "e", "l"],
        ["l", "o", " "],
        ["w", "o", "r"],
        ["l", "d", "!"],
    ];

    for (index, item) in grid.iter_mut().enumerate() {
        item.push(text.chars().nth(index).unwrap());
    }

    for (x, y, item) in grid.enumerate() {
        println!("Item at {}x{}: {}", x, y, item);
    }

    for (x, y, neighbor) in grid.neighbors(1, 1) {
        println!("Neighbor: {}x{}: {}", x, y, neighbor);
        assert_eq!(neighbor, neighbors[y][x]);
    }

    assert_eq!(grid.neighbors(0, 0).count(), 3);
    assert_eq!(grid.neighbors(0, 1).count(), 5);
    assert_eq!(grid.neighbors(0, 2).count(), 5);
    assert_eq!(grid.neighbors(0, 3).count(), 3);
    assert_eq!(grid.neighbors(1, 0).count(), 5);
    assert_eq!(grid.neighbors(1, 1).count(), 8);
    assert_eq!(grid.neighbors(1, 2).count(), 8);
    assert_eq!(grid.neighbors(1, 3).count(), 5);
    assert_eq!(grid.neighbors(2, 0).count(), 3);
    assert_eq!(grid.neighbors(2, 1).count(), 5);
    assert_eq!(grid.neighbors(2, 2).count(), 5);
    assert_eq!(grid.neighbors(2, 3).count(), 3);
}
