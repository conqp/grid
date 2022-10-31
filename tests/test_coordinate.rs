use grid::Coordinate;

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
