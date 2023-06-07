use grid2d::Coordinate;
use grid2d::Grid;

#[test]
fn storage() {
    let mut grid = Grid::new(3, 4, String::new);
    let text = "Hello world!";

    for (index, item) in grid.iter_mut().enumerate() {
        item.push(text.as_bytes()[index] as char);
    }

    assert_eq!(grid.get(Coordinate::new(0, 0)).unwrap(), "H");
    assert_eq!(grid.get(Coordinate::new(1, 0)).unwrap(), "e");
    assert_eq!(grid.get(Coordinate::new(2, 0)).unwrap(), "l");
    assert_eq!(grid.get(Coordinate::new(0, 1)).unwrap(), "l");
    assert_eq!(grid.get(Coordinate::new(1, 1)).unwrap(), "o");
    assert_eq!(grid.get(Coordinate::new(2, 1)).unwrap(), " ");
    assert_eq!(grid.get(Coordinate::new(0, 2)).unwrap(), "w");
    assert_eq!(grid.get(Coordinate::new(1, 2)).unwrap(), "o");
    assert_eq!(grid.get(Coordinate::new(2, 2)).unwrap(), "r");
    assert_eq!(grid.get(Coordinate::new(0, 3)).unwrap(), "l");
    assert_eq!(grid.get(Coordinate::new(1, 3)).unwrap(), "d");
    assert_eq!(grid.get(Coordinate::new(2, 3)).unwrap(), "!");
}

#[test]
fn access_by_tuple() {
    let grid = Grid::try_from(("Hello world!".chars(), 4)).unwrap();
    assert_eq!(grid.get((0, 2)).unwrap(), &'r');
}
