use super::{
  GNum,
  Vector,
  Point,
};

pub trait BaseVectorPoint {
  fn x(&self) -> GNum;
  fn y(&self) -> GNum;
  fn x_mut(&mut self) -> &mut GNum;
  fn y_mut(&mut self) -> &mut GNum;
}

impl BaseVectorPoint for Vector {
  fn x(&self)         ->      GNum { self.x }
  fn y(&self)         ->      GNum { self.y }
  fn x_mut(&mut self) -> &mut GNum { &mut self.x }
  fn y_mut(&mut self) -> &mut GNum { &mut self.y }
}

impl BaseVectorPoint for Point {
  fn x(&self)         ->      GNum { self.x }
  fn y(&self)         ->      GNum { self.y }
  fn x_mut(&mut self) -> &mut GNum { &mut self.x }
  fn y_mut(&mut self) -> &mut GNum { &mut self.y }
}
