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
    assert_eq!(grid.len(), width * height);
}

#[test]
fn storage() {
    let mut grid = Grid::new(3, 4, String::new);
    let text = "Hello world!";

    for (index, item) in grid.iter_mut().enumerate() {
        item.push(text.chars().nth(index).unwrap());
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
