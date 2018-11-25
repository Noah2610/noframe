use super::Point;

#[test]
fn set_position() {
  let mut point = Point::new(0.0, 0.0);
  let point_new = Point::new(10.0, 20.0);
  point.set(&point_new);
  assert_eq!(point, point_new);
}

#[test]
fn add_position() {
  let mut point = Point::new(10.0, 20.0);
  let point_add = Point::new(5.0, 15.0);
  point.add(&point_add);
  assert_eq!(point.as_tup(), (15.0, 35.0));
}

#[test]
fn combine_multiple_points() {
  let points = vec![
    Point::new( 5.0,   10.0),
    Point::new( 15.0,  7.0),
    Point::new( 42.0,  24.0),
    Point::new(-21.0, -22.0)
  ];
  let point = Point::combine(
    points.iter().map( |p| p ).collect()  // Create a `Vec<&Point>` from a `Vec<Point>`
  );
  assert_eq!(point.as_tup(), (41.0, 19.0));
}

#[test]
fn create_ggez_point2_from_point() {
  let point  = Point::new(10.0, 20.0);
  let point2 = ::ggez::graphics::Point2::from(&point);
  assert_eq!(point.as_tup(), (point2.x, point2.y));
}
