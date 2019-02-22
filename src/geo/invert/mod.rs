use super::{
  GNum,
  Vector,
  Point,
  BaseVectorPoint,
};

pub trait Invert: BaseVectorPoint + Sized + Clone {
  fn invert(&mut self) {
    let negone = GNum::from(-1i8);
    *self.x_mut() *= negone;
    *self.y_mut() *= negone;
  }

  fn inverted(&self) -> Self {
    let mut copy = self.clone();
    copy.invert();
    copy
  }
}

impl Invert for Vector { }
impl Invert for Point { }

#[cfg(test)]
mod tests;
