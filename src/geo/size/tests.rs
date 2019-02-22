use super::super::{ Vector, Size };

#[test]
fn center_of_size() {
  let size = Size::new(20.0, 10.0);
  assert_eq!(size.center(), Vector::new(10.0, 5.0));
}
