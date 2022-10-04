use grid::Coordinate;
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

    for (coordinate, item) in grid.enumerate() {
        println!("Item at {}: {}", coordinate, item);
    }

    for (coordinate, neighbor) in grid.neighbors(Coordinate::new(1, 1)) {
        println!("Neighbor: {}: {}", coordinate, neighbor);
        assert_eq!(neighbor, neighbors[coordinate.y()][coordinate.x()]);
    }

    assert_eq!(grid.neighbors(Coordinate::new(0, 0)).count(), 3);
    assert_eq!(grid.neighbors(Coordinate::new(0, 1)).count(), 5);
    assert_eq!(grid.neighbors(Coordinate::new(0, 2)).count(), 5);
    assert_eq!(grid.neighbors(Coordinate::new(0, 3)).count(), 3);
    assert_eq!(grid.neighbors(Coordinate::new(1, 0)).count(), 5);
    assert_eq!(grid.neighbors(Coordinate::new(1, 1)).count(), 8);
    assert_eq!(grid.neighbors(Coordinate::new(1, 2)).count(), 8);
    assert_eq!(grid.neighbors(Coordinate::new(1, 3)).count(), 5);
    assert_eq!(grid.neighbors(Coordinate::new(2, 0)).count(), 3);
    assert_eq!(grid.neighbors(Coordinate::new(2, 1)).count(), 5);
    assert_eq!(grid.neighbors(Coordinate::new(2, 2)).count(), 5);
    assert_eq!(grid.neighbors(Coordinate::new(2, 3)).count(), 3);
}

#[test]
fn coordinate_from_tuple() {
    let coordinate: Coordinate = (32, 1337).into();
    assert_eq!(Coordinate::new(32, 1337), coordinate);
}

#[test]
fn coordinate_from_tuple_ref() {
    let tuple = (32, 1337);
    let tuple_ref = &tuple;
    let coordinate: Coordinate = tuple_ref.into();
    assert_eq!(Coordinate::new(32, 1337), coordinate);
}

#[test]
fn tuple_from_coordinate() {
    let (x, y) = Coordinate::new(32, 1337).into();
    assert_eq!((32, 1337), (x, y));
}

#[test]
fn tuple_from_coordinate_ref() {
    let coordinate = Coordinate::new(32, 1337);
    let coordinate_ref = &coordinate;
    let (x, y) = coordinate_ref.into();
    assert_eq!((32, 1337), (x, y));
}

#[test]
fn access_by_tuple() {
    let grid = Grid::from_iter("Hello world!".chars(), 4).unwrap();
    assert_eq!(grid.get((0, 2)).unwrap(), &'r');
}
