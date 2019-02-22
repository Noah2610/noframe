pub type GNum = f32;

pub mod size;
pub mod mask;
pub mod rect;
pub mod base_vector_point;
pub mod invert;
pub mod conversions;

pub mod prelude {
  pub use super::GNum;
  pub use super::Vector;
  pub use super::Point;
  pub use super::Size;
  pub use super::Mask;
  pub use super::Origin;
  pub use super::Rect;
  pub use super::BaseVectorPoint;
  pub use super::Invert;
  pub use super::{ AsVector, AsPoint };
}

pub use self::size::Size;
pub use self::mask::Mask;
pub use self::mask::Origin;
pub use self::rect::Rect;
pub use self::base_vector_point::BaseVectorPoint;
pub use self::invert::Invert;
pub use self::conversions::{ AsVector, AsPoint };

use nalgebra::Vector2;
use nalgebra::Point2;
pub type Vector = Vector2<GNum>;
pub type Point  = Point2<GNum>;
