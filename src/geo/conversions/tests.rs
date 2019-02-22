use super::super::{
  Vector,
  Point,
  conversions::*,
};

#[test]
fn vector_to_point() {
  let vector = Vector::new(10.0, 20.0);
  assert_eq!(Point::new(10.0, 20.0), vector.as_point());
}

#[test]
fn point_to_vector() {
  let point = Point::new(10.0, 20.0);
  assert_eq!(Vector::new(10.0, 20.0), point.as_vector());
}
