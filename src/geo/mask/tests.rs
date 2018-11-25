use super::super::NumType;
use super::super::prelude::*;

#[derive(Debug)]
struct Body {
  point:  Point,
  size:   Size,
  origin: Origin
}

impl Body {
  pub fn new(x: NumType, y: NumType, w: NumType, h: NumType) -> Self {
    Self {
      point:  Point::new(x, y),
      size:   Size::new(w, h),
      origin: Origin::TopLeft
    }
  }
}

impl Mask for Body {
  fn point(&self) -> &Point {
    &self.point
  }
  fn point_mut(&mut self) -> &mut Point {
    &mut self.point
  }
  fn size(&self) -> &Size {
    &self.size
  }
  fn origin(&self) -> &Origin {
    &self.origin
  }
}

fn get_not_intersecting_bodies() -> (Body, Body) {
  // -----
  // |ONE|
  // ---------
  //     |TWO|
  //     -----
  (
    Body::new(10.0, 10.0, 10.0, 10.0),
    Body::new(20.0, 20.0, 10.0, 10.0)
  )
}

fn get_intersecting_bodies() -> (Body, Body) {
  // -----
  // |ONE|--
  // --|TWO|
  //   -----
  (
    Body::new(10.0, 10.0, 10.0, 10.0),
    Body::new(15.0, 15.0, 10.0, 10.0)
  )
}

#[test]
fn do_not_collide() {
  let bodies = get_not_intersecting_bodies();
  assert!(!bodies.0.intersects(&bodies.1), "Should NOT collide:\n{:#?}\n", &bodies);
}

#[test]
fn do_collide() {
  let bodies = get_intersecting_bodies();
  assert!(bodies.0.intersects(&bodies.1), "SHOULD collide:\n{:#?}\n", &bodies);
}
