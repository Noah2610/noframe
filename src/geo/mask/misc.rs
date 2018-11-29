#[derive(Debug, Clone)]
pub enum Origin {
  TopLeft,
  TopRight,
  TopCenter,
  BottomLeft,
  BottomRight,
  BottomCenter,
  CenterLeft,
  CenterRight,
  Center
}

#[derive(Debug)]
pub enum Side {
  Top,
  Bottom,
  Left,
  Right
}

#[derive(PartialEq)]
pub struct SideCollection {
  pub top:    f32,
  pub bottom: f32,
  pub left:   f32,
  pub right:  f32
}

impl SideCollection {
  pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {
    Self {
      top,
      bottom,
      left,
      right
    }
  }

  pub fn round(&self) -> Self {
    Self::new(
      self.top.round(),
      self.bottom.round(),
      self.left.round(),
      self.right.round()
    )
  }
}
