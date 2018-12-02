use ::ggez::{
  GameResult,
  Context
};

use ::geo::{
  NumType,
  point::Point,
  size::Size,
  mask::Mask,
  mask::misc::Origin
};
use ::entity::Entity;

pub struct Camera {
  point:  Point,
  size:   Size,
  origin: Origin
}

impl Camera {
  pub fn new(size: Size) -> Self {
    Camera {
      point: Point::new(0.0, 0.0),
      size,
      origin: Origin::Center
    }
  }

  pub fn point(&self) -> &Point {
    &self.point
  }

  pub fn move_to(&mut self, point: &Point) {
    self.point.set(point);
  }

  pub fn move_by(&mut self, point: &Point) {
    self.point.add(point);
  }

  pub fn move_x(&mut self, incr: NumType) {
    self.move_by(&Point::new(incr, 0.0));
  }

  pub fn move_y(&mut self, incr: NumType) {
    self.move_by(&Point::new(0.0, incr));
  }

  pub fn draw<E: Entity>(&self, ctx: &mut Context, entity: &E) -> GameResult<()> {
    if self.intersects(entity) {
      entity.draw_offset(ctx, &self.top_left().inverted())?;
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
