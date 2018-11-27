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

#[cfg(test)]
mod tests;
