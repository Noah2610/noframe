use super::NumType;
use super::point::Point;

#[derive(Debug, Clone)]
pub struct Size {
  pub w: NumType,
  pub h: NumType
}

impl Size {
  pub fn new(w: NumType, h: NumType) -> Self {
    Self { w, h }
  }

  pub fn center(&self) -> Point {
    Point::new(self.w / 2.0, self.h / 2.0)
  }
}

impl From<[NumType; 2]> for Size {
  fn from(arr: [NumType; 2]) -> Self {
    Self::new(arr[0], arr[1])
  }
}

impl From<(NumType, NumType)> for Size {
  fn from(tup: (NumType, NumType)) -> Self {
    Self::new(tup.0, tup.1)
  }
}

#[cfg(test)]
mod tests;
