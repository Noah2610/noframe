pub mod traits;

pub mod prelude;
pub use self::traits::velocity::Velocity;
pub use self::traits::movement::Movement;

use ::ggez::{
  Context,
  GameResult,
  graphics
};

use ::settings::entity::*;

use ::color::Color;
use ::geo::{
  point::Point,
  size::Size,
  mask::Mask
};

pub trait Entity: Mask {
  fn color(&self) -> Color {
    DEFAULT_COLOR
  }

  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }

  fn draw_rect(&mut self, ctx: &mut Context, rect: [f32; 4]) -> GameResult<()> {
    graphics::set_color(ctx, self.color().into())?;
    return graphics::rectangle(ctx, graphics::DrawMode::Fill, rect.into());
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    let rect = {
      let point: Point = self.top_left();
      let size:  &Size = self.size();
      [
        point.x, point.y,
        size.w,  size.h
      ]
    };
    return self.draw_rect(ctx, rect);
  }

  fn draw_offset(&mut self, ctx: &mut Context, offset: &Point) -> GameResult<()> {
    let rect = {
      let point: Point = Point::combine(vec![&self.top_left(), offset]);
      let size:  &Size = self.size();
      [
        point.x, point.y,
        size.w,  size.h
      ]
    };
    return self.draw_rect(ctx, rect);
  }
}
