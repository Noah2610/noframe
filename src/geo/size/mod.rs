use std::fmt::Debug;
use std::cmp::PartialEq;

use super::Vector;
use super::num_traits::*;

#[derive(Debug, Clone)]
pub struct Size<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd + 'static {
  pub w: T,
  pub h: T,
}

impl<T> Size<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd + 'static {
  pub fn new(w: T, h: T) -> Self {
    Self { w, h }
  }

  pub fn center(&self) -> Vector<T> {
    let two = T::one() + T::one();
    Vector::new(self.w / two, self.h / two)
  }
}

impl<T> From<[T; 2]> for Size<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd + 'static {
  fn from(arr: [T; 2]) -> Self {
    Self::new(arr[0], arr[1])
  }
}

impl<T> From<(T, T)> for Size<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd + 'static {
  fn from(tup: (T, T)) -> Self {
    Self::new(tup.0, tup.1)
  }
}

#[cfg(test)]
mod tests;
