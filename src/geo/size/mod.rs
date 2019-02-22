use super::GNum;
use super::Vector;

#[derive(Debug, Clone)]
pub struct Size {
  pub w: GNum,
  pub h: GNum,
}

impl Size {
  pub fn new(w: GNum, h: GNum) -> Self {
    Self { w, h }
  }

  pub fn center(&self) -> Vector {
    Vector::new(self.w / GNum::from(2i8), self.h / GNum::from(2i8))
  }
}

impl From<[GNum; 2]> for Size {
  fn from(arr: [GNum; 2]) -> Self {
    Self::new(arr[0], arr[1])
  }
}

impl From<(GNum, GNum)> for Size {
  fn from(tup: (GNum, GNum)) -> Self {
    Self::new(tup.0, tup.1)
  }
}

#[cfg(test)]
mod tests;
