pub type NumType = f32;

pub mod point;
pub mod size;
pub mod mask;
pub mod rect;

pub mod prelude;

pub mod num_traits {
  pub use num_traits::{
    Num,
    NumOps,
    NumAssignOps,
    Float,
    Signed,
    ToPrimitive,
    FromPrimitive,
    NumCast,
  };
}
