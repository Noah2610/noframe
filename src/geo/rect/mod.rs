use super::{
  GNum,
  Point,
  Size,
  Mask,
  Origin,
};

#[derive(Debug)]
pub struct Rect {
  point:  Point,
  size:   Size,
  origin: Origin,
}

impl Rect {
  pub fn new(point: Point, size: Size, origin: Origin) -> Self {
    Self {
      point,
      size,
      origin
    }
  }

  pub fn new_simple(x: GNum, y: GNum, w: GNum, h: GNum) -> Self {
    Self {
      point:  Point::new(x, y),
      size:   Size::new(w, h),
      origin: Origin::TopLeft
    }
  }
}

impl Mask for Rect {
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
