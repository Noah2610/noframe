pub mod traits;

pub mod prelude {
  pub use super::Entity;
  pub use super::traits::{
    velocity::Velocity,
    movement::Movement,
  };
}
pub use self::traits::velocity::Velocity;
pub use self::traits::movement::Movement;

use ggez::{
  Context,
  GameResult,
  graphics::{ self, Mesh, DrawMode, DrawParam },
};

use crate::settings::entity::*;
use crate::color::Color;
use crate::geo::{
  Vector,
  Mask,
};

pub trait Entity: Mask {
  fn color(&self) -> Color {
    DEFAULT_COLOR
  }

  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }

  fn draw_rect(&self, ctx: &mut Context, rect: graphics::Rect) -> GameResult<()> {
    // let rect = [
    //   rect[0].round(),
    //   rect[1].round(),
    //   rect[2].round(),
    //   rect[3].round(),
    // ];
    let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, self.color().into())?;
    graphics::draw(ctx, &mesh, DrawParam::new())
  }

  fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    let point = self.top_left();
    let size  = self.size();
    let rect = graphics::Rect::new(
      point.x, point.y,
      size.w,  size.h
    );
    self.draw_rect(ctx, rect)
  }

  fn draw_offset(&self, ctx: &mut Context, offset: &Vector) -> GameResult<()> {
    let point = self.top_left() + offset;
    let size  = self.size();
    let rect = graphics::Rect::new(
      point.x, point.y,
      size.w,  size.h
    );
    return self.draw_rect(ctx, rect);
  }
}
