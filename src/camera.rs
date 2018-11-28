use ::ggez::{
  GameResult,
  Context
};

use ::geo::{
  NumType,
  point::Point
};
use ::entity::Entity;

pub struct Camera {
  point: Point
}

impl Camera {
  pub fn new() -> Self {
    Camera { point: Point::new(0.0, 0.0) }
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
    entity.draw_offset(ctx, &self.point.inverted())
  }
}
