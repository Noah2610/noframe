use std::fmt::Debug;

use super::{
  Point,
  Size,
  Mask,
  Origin,
  num_traits::*,
};

#[derive(Debug)]
pub struct Rect<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd + 'static {
  point:  Point<T>,
  size:   Size<T>,
  origin: Origin,
}

impl<T> Rect<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd + 'static {
  pub fn new(point: Point<T>, size: Size<T>, origin: Origin) -> Self {
    Self {
      point,
      size,
      origin
    }
  }

  pub fn new_simple(x: T, y: T, w: T, h: T) -> Self {
    Self {
      point:  Point::new(x, y),
      size:   Size::new(w, h),
      origin: Origin::TopLeft
    }
  }
}

impl<T> Mask<T> for Rect<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd + 'static {
  fn point(&self) -> &Point<T> {
    &self.point
  }
  fn point_mut(&mut self) -> &mut Point<T> {
    &mut self.point
  }
  fn size(&self) -> &Size<T> {
    &self.size
  }
  fn origin(&self) -> &Origin {
    &self.origin
  }
}
