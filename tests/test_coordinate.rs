use grid::Coordinate;
use grid::CoordinateParseError;
use std::str::FromStr;

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
fn parse_coordinate() {
    let coordinate = Coordinate::from_str("-1 1");
    assert!(coordinate.is_err());
    assert_eq!(coordinate.unwrap_err(), CoordinateParseError::InvalidXValue);

    let coordinate = Coordinate::from_str("1 -1");
    assert!(coordinate.is_err());
    assert_eq!(coordinate.unwrap_err(), CoordinateParseError::InvalidYValue);

    let coordinate = Coordinate::from_str("a 42");
    assert!(coordinate.is_err());
    assert_eq!(coordinate.unwrap_err(), CoordinateParseError::InvalidXValue);

    let coordinate = Coordinate::from_str("42 a");
    assert!(coordinate.is_err());
    assert_eq!(coordinate.unwrap_err(), CoordinateParseError::InvalidYValue);

    let coordinate = Coordinate::from_str("42");
    assert!(coordinate.is_err());
    assert_eq!(coordinate.unwrap_err(), CoordinateParseError::NotTwoNumbers);

    let coordinate = Coordinate::from_str(" 42");
    assert!(coordinate.is_err());
    assert_eq!(coordinate.unwrap_err(), CoordinateParseError::InvalidXValue);

    let coordinate = Coordinate::from_str("abc");
    assert!(coordinate.is_err());
    assert_eq!(coordinate.unwrap_err(), CoordinateParseError::NotTwoNumbers);

    let coordinate = Coordinate::from_str("42 ");
    assert!(coordinate.is_err());
    assert_eq!(coordinate.unwrap_err(), CoordinateParseError::InvalidYValue);

    let coordinate = Coordinate::from_str("42 1337");
    assert!(coordinate.is_ok());
    assert_eq!(coordinate.unwrap(), Coordinate::new(42, 1337));

    let coordinate = Coordinate::from_str("0 0");
    assert!(coordinate.is_ok());
    assert_eq!(coordinate.unwrap(), Coordinate::new(0, 0));
}
