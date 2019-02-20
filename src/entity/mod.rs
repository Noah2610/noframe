pub mod traits;

pub mod prelude;
pub use self::traits::velocity::Velocity;
pub use self::traits::movement::Movement;

use std::fmt::Debug;

use ::ggez::{
  Context,
  GameResult,
  graphics::{ self, Mesh, DrawMode, DrawParam },
};

use ::settings::entity::*;
use ::color::Color;
use ::geo::{
  Point,
  Size,
  Mask,
  num_traits::*,
};

pub trait Entity<T>: Mask<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd {
  fn color(&self) -> Color {
    DEFAULT_COLOR
  }

  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }

  fn draw_rect(&self, ctx: &mut Context, rect: [T; 4]) -> GameResult<()> {
    // let rect = [
    //   rect[0].round(),
    //   rect[1].round(),
    //   rect[2].round(),
    //   rect[3].round()
    // ];
    let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect.into(), self.color().into())?;
    graphics::draw(ctx, &mesh, DrawParam::new())
  }

  fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    let rect = {
      let point: Point<T> = self.top_left();
      let size:  &Size<T> = self.size();
      [
        point.x, point.y,
        size.w,  size.h
      ]
    };
    self.draw_rect(ctx, rect)
  }

  fn draw_offset(&self, ctx: &mut Context, offset: &Point<T>) -> GameResult<()> {
    let rect = {
      let point: Point<T> = Point::combine(vec![&self.top_left(), offset]);
      let size:  &Size<T> = self.size();
      [
        point.x, point.y,
        size.w,  size.h
      ]
    };
    return self.draw_rect(ctx, rect);
  }
}
