use super::super::GNum;

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
  pub top:    GNum,
  pub bottom: GNum,
  pub left:   GNum,
  pub right:  GNum
}

impl SideCollection {
  pub fn new(top: GNum, bottom: GNum, left: GNum, right: GNum) -> Self {
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
  type Item = (Side, GNum);

  fn next(&mut self) -> Option<(Side, GNum)> {
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

impl std::iter::FromIterator<GNum> for SideCollection {
  fn from_iter<T: IntoIterator<Item=GNum>>(iter: T) -> Self {
    let zero = GNum::from(0i8);
    let mut iter = iter.into_iter();
    Self::new(
      iter.next().unwrap_or(zero),
      iter.next().unwrap_or(zero),
      iter.next().unwrap_or(zero),
      iter.next().unwrap_or(zero),
    )
  }
}
