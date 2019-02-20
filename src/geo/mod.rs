pub mod size;
pub mod mask;
pub mod rect;

pub mod prelude {
  pub use super::Vector;
  pub use super::Point;
  pub use super::Size;
  pub use super::Mask;
  pub use super::Origin;
  pub use super::Rect;
  pub use num_traits::*;
}

pub mod num_traits {
  pub use num_traits::Num;
  pub use num_traits::PrimInt as Integer;
  pub use num_traits::Float;
  pub use num_traits::Signed;
}

pub use nalgebra::Vector2 as Vector;
pub use nalgebra::Point2 as Point;
pub use self::size::Size;
pub use self::mask::Mask;
pub use self::mask::misc::Origin;
pub use self::rect::Rect;
