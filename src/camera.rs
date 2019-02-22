use ggez::{
  GameResult,
  Context
};

use crate::geo::{
  GNum,
  Vector,
  Point,
  Size,
  Mask,
  Origin,
  Invert,
  conversions::*,
};
use crate::entity::Entity;

pub struct Camera {
  point:  Point,
  size:   Size,
  origin: Origin,
}

impl Camera {
  pub fn new(size: Size) -> Self {
    let zero = GNum::from(0i8);
    Camera {
      point: Point::new(zero, zero),
      size,
      origin: Origin::Center
    }
  }

  pub fn point(&self) -> &Point {
    &self.point
  }

  pub fn move_to(&mut self, point: &Point) {
    self.point = *point;
  }

  pub fn move_by(&mut self, vector: &Vector) {
    self.point += vector;
  }

  pub fn move_x(&mut self, incr: GNum) {
    self.point.x += incr;
  }

  pub fn move_y(&mut self, incr: GNum) {
    self.point.y += incr;
  }

  pub fn draw<E: Entity>(&self, ctx: &mut Context, entity: &E) -> GameResult<()> {
    if self.intersects(entity) {
      let vector = self.top_left().inverted().as_vector();
      entity.draw_offset(ctx, &vector)?;
    }
    Ok(())
  }
}

impl Mask for Camera {
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
