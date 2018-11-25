use super::NumType;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
  pub x: NumType,
  pub y: NumType
}

impl Point {
  pub fn new(x: NumType, y: NumType) -> Self {
    Self { x, y }
  }

  pub fn combine(points: Vec<&Point>) -> Point {
    let mut point_acc: Point = Point::new(0.0, 0.0);
    for point in points {
      point_acc.add(point);
    }
    return point_acc;
  }

  pub fn set(&mut self, point: &Point) {
    self.x = point.x;
    self.y = point.y;
  }

  pub fn add(&mut self, point: &Point) {
    self.x += point.x;
    self.y += point.y;
  }

  pub fn as_tup(&self) -> (NumType, NumType) {
    (self.x, self.y)
  }
}

/// Implement the `From` trait for ggez's `Point2` struct,
/// so that ggez's `Point2` struct can easily be created from our `Point` struct.
impl<'a> From<&'a Point> for ::ggez::graphics::Point2 {
  fn from(point: &Point) -> Self {
    Self::new(point.x, point.y)
  }
}

#[cfg(test)]
mod tests;
