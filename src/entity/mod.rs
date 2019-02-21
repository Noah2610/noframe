pub mod traits;

pub mod prelude;
pub use self::traits::velocity::Velocity;
pub use self::traits::movement::Movement;

use std::fmt::Debug;
use std::ops::AddAssign;

use ::ggez::{
  Context,
  GameResult,
  graphics::{ self, Mesh, DrawMode, DrawParam },
};

use ::settings::entity::*;
use ::color::Color;
use ::geo::{
  Vector,
  Point,
  Size,
  Mask,
  num_traits::*,
};

pub trait Entity<T>: Mask<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd + AddAssign + Into<f32> + 'static {
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
    //   rect[3].round()
    // ];
    let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, self.color().into())?;
    graphics::draw(ctx, &mesh, DrawParam::new())
  }

  fn draw(&self, ctx: &mut Context) -> GameResult<()> {
    let point = self.top_left();
    let size  = self.size();
    let rect = graphics::Rect::new(point.x.into(), point.y.into(), size.w.into(), size.h.into());
    self.draw_rect(ctx, rect)
  }

  fn draw_offset(&self, ctx: &mut Context, offset: &Vector<T>) -> GameResult<()> {
    let point = self.top_left() + offset;
    let size  = self.size();
    let rect = graphics::Rect::new(point.x.into(), point.y.into(), size.w.into(), size.h.into());
    return self.draw_rect(ctx, rect);
  }
}
