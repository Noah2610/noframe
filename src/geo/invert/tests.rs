use super::super::{
  Vector,
  Point,
  Invert,
};

#[test]
fn invert_vector() {
  let mut vector = Vector::new(10.0, 20.0);
  vector.invert();
  assert_eq!(Vector::new(-10.0, -20.0), vector);
}

#[test]
fn inverted_vector() {
  let vector = Vector::new(10.0, 20.0);
  assert_eq!(Vector::new(-10.0, -20.0), vector.inverted());
}

#[test]
fn invert_point() {
  let mut point = Point::new(10.0, 20.0);
  point.invert();
  assert_eq!(Point::new(-10.0, -20.0), point);
}

#[test]
fn inverted_point() {
  let point = Point::new(10.0, 20.0);
  assert_eq!(Point::new(-10.0, -20.0), point.inverted());
}
