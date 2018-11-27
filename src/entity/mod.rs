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
use ::geo::prelude::*;

pub trait Entity: Mask {
  fn color(&self) -> Color {
    DEFAULT_COLOR
  }

  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::set_color(ctx, self.color().into())?;
    graphics::rectangle(ctx, graphics::DrawMode::Fill, self.as_rect())?;
    return Ok(());
  }
}
