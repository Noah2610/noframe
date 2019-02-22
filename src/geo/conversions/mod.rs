use super::{
  Vector,
  Point,
  BaseVectorPoint,
};

pub trait AsVector: BaseVectorPoint {
  fn as_vector(&self) -> Vector {
    Vector::new(self.x(), self.y())
  }
}

pub trait AsPoint: BaseVectorPoint {
  fn as_point(&self) -> Point {
    Point::new(self.x(), self.y())
  }
}

impl AsVector for Point {}
impl AsPoint for Vector {}

#[cfg(test)]
mod tests;
