use std::fmt::Debug;
use std::ops;

use ::ggez::{
  GameResult,
  Context
};

use ::geo::{
  Vector,
  Point,
  Size,
  Mask,
  Origin,
  num_traits::*,
};
use ::entity::Entity;

pub struct Camera<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd + ops::AddAssign + Signed + Into<f32> + 'static {
  point:  Point<T>,
  size:   Size<T>,
  origin: Origin,
}

impl<T> Camera<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd + ops::AddAssign + Signed + Into<f32> + 'static {
  pub fn new(size: Size<T>) -> Self {
    Camera {
      point: Point::new(T::zero(), T::zero()),
      size,
      origin: Origin::Center
    }
  }

  pub fn point(&self) -> &Point<T> {
    &self.point
  }

  pub fn move_to(&mut self, point: &Point<T>) {
    self.point.x = point.x;
    self.point.y = point.y;
  }

  pub fn move_by(&mut self, vector: &Vector<T>) {
    self.point += vector;
  }

  pub fn move_x(&mut self, incr: T) {
    self.move_by(&Vector::new(incr, T::zero()));
  }

  pub fn move_y(&mut self, incr: T) {
    self.move_by(&Vector::new(T::zero(), incr));
  }

  pub fn draw<E: Entity<T>>(&self, ctx: &mut Context, entity: &E) -> GameResult<()> {
    if self.intersects(entity) {
      entity.draw_offset(ctx, &crate::geo::invert_point(&self.top_left()).into())?;
    }
    Ok(())
  }
}

impl<T> Mask<T> for Camera<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd + ops::AddAssign + Signed + 'static {
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
