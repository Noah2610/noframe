use super::{
  NumType,
  point::Point,
  size::Size,
  mask::{
    Mask,
    misc::Origin
  }
};

#[derive(Debug)]
pub struct Rect {
  point:  Point<NumType>,
  size:   Size<NumType>,
  origin: Origin,
}

impl Rect {
  pub fn new(point: Point<NumType>, size: Size<NumType>, origin: Origin) -> Self {
    Self {
      point,
      size,
      origin
    }
  }

  pub fn new_simple(x: NumType, y: NumType, w: NumType, h: NumType) -> Self {
    Self {
      point:  Point::new(x, y),
      size:   Size::new(w, h),
      origin: Origin::TopLeft
    }
  }
}

impl Mask for Rect {
  fn point(&self) -> &Point<NumType> {
    &self.point
  }
  fn point_mut(&mut self) -> &mut Point<NumType> {
    &mut self.point
  }
  fn size(&self) -> &Size<NumType> {
    &self.size
  }
  fn origin(&self) -> &Origin {
    &self.origin
  }
}
