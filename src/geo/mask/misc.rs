use super::super::NumType;

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
  index:      usize,
  pub top:    NumType,
  pub bottom: NumType,
  pub left:   NumType,
  pub right:  NumType
}

impl SideCollection {
  pub fn new(top: NumType, bottom: NumType, left: NumType, right: NumType) -> Self {
    Self {
      index: 0,
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

impl Iterator for SideCollection {
  type Item = (Side, NumType);

  fn next(&mut self) -> Option<(Side, NumType)> {
    self.index += 1;
    match self.index {
      1 => Some((Side::Top,    self.top)),
      2 => Some((Side::Bottom, self.bottom)),
      3 => Some((Side::Left,   self.left)),
      4 => Some((Side::Right,  self.right)),
      _ => None
    }
  }
}

impl std::iter::FromIterator<NumType> for SideCollection {
  fn from_iter<T: IntoIterator<Item=NumType>>(iter: T) -> Self {
    let mut iter = iter.into_iter();
    Self::new(
      iter.next().unwrap_or(0.0),
      iter.next().unwrap_or(0.0),
      iter.next().unwrap_or(0.0),
      iter.next().unwrap_or(0.0),
    )
  }
}
