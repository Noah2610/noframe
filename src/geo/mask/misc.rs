use std::fmt::Debug;

use super::super::num_traits::*;

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
pub struct SideCollection<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd {
  index:      usize,
  pub top:    T,
  pub bottom: T,
  pub left:   T,
  pub right:  T
}

impl<T> SideCollection<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd {
  pub fn new(top: T, bottom: T, left: T, right: T) -> Self {
    Self {
      index: 0,
      top,
      bottom,
      left,
      right
    }
  }

  pub fn round(&self) -> Self where T: Float {
    Self::new(
      self.top.round(),
      self.bottom.round(),
      self.left.round(),
      self.right.round()
    )
  }
}

impl<T> Iterator for SideCollection<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd {
  type Item = (Side, T);

  fn next(&mut self) -> Option<(Side, T)> {
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

impl<T> std::iter::FromIterator<T> for SideCollection<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd {
  fn from_iter<U: IntoIterator<Item=T>>(iter: U) -> Self {
    let mut iter = iter.into_iter();
    Self::new(
      iter.next().unwrap_or(T::zero()),
      iter.next().unwrap_or(T::zero()),
      iter.next().unwrap_or(T::zero()),
      iter.next().unwrap_or(T::zero()),
    )
  }
}
