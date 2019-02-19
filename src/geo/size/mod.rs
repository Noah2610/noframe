use super::NumType;
use super::point::Point;
use super::num_traits::*;

#[derive(Debug, Clone)]
pub struct Size<T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast> {
  pub w: T,
  pub h: T,
}

impl<T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast> Size<T> {
  pub fn new(w: T, h: T) -> Self {
    Self { w, h }
  }

  pub fn center(&self) -> Point<T> {
    Point::new(self.w / T::from(2).unwrap(), self.h / T::from(2).unwrap())
  }
}

impl<T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast> From<[T; 2]> for Size<T> {
  fn from(arr: [T; 2]) -> Self {
    Self::new(arr[0], arr[1])
  }
}

impl<T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast> From<(T, T)> for Size<T> {
  fn from(tup: (T, T)) -> Self {
    Self::new(tup.0, tup.1)
  }
}

#[cfg(test)]
mod tests;
