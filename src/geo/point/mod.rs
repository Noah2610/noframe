use std::{ ops, fmt };
use super::NumType;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
  pub x: NumType,
  pub y: NumType
}

impl Point {
  /// Returns a new `Point` with the passed `x` and `y` values.
  /// # Example
  ///   ```
  ///   use noframe::geo::point::Point;
  ///
  ///   let point = Point::new(32.0, 64.0);
  ///
  ///   assert_eq!((32.0, 64.0), (point.x, point.y))
  ///   ```
  pub fn new(x: NumType, y: NumType) -> Self {
    Self { x, y } }

  /// Returns a new `Point` with the accumulated `x` and `y` values
  /// of all points, passed as a `Vec<&Point>`.
  /// # Example
  ///   ```
  ///   use noframe::geo::point::Point;
  ///
  ///   let point1 = Point::new(10.0, 2.0);
  ///   let point2 = Point::new(11.0, 3.0);
  ///   let point3 = Point::new(21.0, 5.0);
  ///   let points = vec![ &point1, &point2, &point3 ];
  ///
  ///   assert_eq!(Point::new(42.0, 10.0), Point::combine(points));
  ///   ```
  pub fn combine(points: Vec<&Point>) -> Point {
    let mut point_acc: Point = Point::new(0.0, 0.0);
    for point in points {
      point_acc.add(point);
    }
    return point_acc;
  }

  /// Set the `x` and `y` values to the values of the passed `Point` reference.
  pub fn set(&mut self, point: &Point) {
    self.x = point.x;
    self.y = point.y;
  }

  /// Set the `x` value.
  pub fn set_x(&mut self, value: NumType) {
    self.x = value;
  }

  /// Set the `y` value.
  pub fn set_y(&mut self, value: NumType) {
    self.y = value;
  }

  /// Add the `x` and `y` values of the passed `Point` to this `Point`.
  /// # Example
  ///   ```
  ///   use noframe::geo::point::Point;
  ///
  ///   let mut point = Point::new(5.0, 15.0);
  ///   let point_add = Point::new(10.0, -5.0);
  ///   point.add(&point_add);
  ///
  ///   assert_eq!(Point::new(15.0, 10.0), point);
  ///   ```
  pub fn add(&mut self, point: &Point) {
    self.x += point.x;
    self.y += point.y;
  }

  /// Return the point's values as a tuple, like `(x, y)`.
  /// # Example
  ///   ```
  ///   use noframe::geo::point::Point;
  ///
  ///   let point = Point::new(10.0, 20.0);
  ///
  ///   assert_eq!((10.0, 20.0), point.as_tup());
  ///   ```
  pub fn as_tup(&self) -> (NumType, NumType) {
    (self.x, self.y)
  }

  /// Returns a new `Point` with the `x` and `y` values multiplied by `-1`.
  /// # Example
  ///   ```
  ///   use noframe::geo::point::Point;
  ///
  ///   let point = Point::new(10.0, 20.0);
  ///
  ///   assert_eq!(Point::new(-10.0, -20.0), point.inverted());
  ///   ```
  pub fn inverted(&self) -> Point {
    Point::new(self.x * -1.0, self.y * -1.0)
  }

  /// Multiplies the `x` and `y` values by `-1`.
  /// # Example
  ///   ```
  ///   use noframe::geo::point::Point;
  ///
  ///   let mut point = Point::new(10.0, 20.0);
  ///   point.invert();
  ///
  ///   assert_eq!(Point::new(-10.0, -20.0), point);
  ///   ```
  pub fn invert(&mut self) {
    self.x *= -1.0;
    self.y *= -1.0;
  }

  pub fn mult_axes_by(&self, mult: NumType) -> Point {
    Point::new(
      self.x * mult,
      self.y * mult
    )
  }
}

impl ops::Add for Point {
  type Output = Point;
  fn add(self, other: Point) -> Point {
    Point::combine(vec![&self, &other])
  }
}

impl ops::AddAssign for Point {
  fn add_assign(&mut self, other: Point) {
    self.x += other.x;
    self.y += other.y;
  }
}

impl ops::Sub for Point {
  type Output = Point;
  fn sub(self, other: Point) -> Point {
    Point::combine(vec![&self, &other.inverted()])
  }
}

impl ops::SubAssign for Point {
  fn sub_assign(&mut self, other: Point) {
    self.x -= other.x;
    self.y -= other.y;
  }
}

impl ops::Mul for Point {
  type Output = Point;
  fn mul(self, other: Point) -> Point {
    Point::new(
      self.x * other.x,
      self.y * other.y
    )
  }
}

impl ops::MulAssign for Point {
  fn mul_assign(&mut self, other: Point) {
    self.x *= other.x;
    self.y *= other.y;
  }
}

impl ops::Div for Point {
  type Output = Point;
  fn div(self, other: Point) -> Point {
    Point::new(
      self.x / other.x,
      self.y / other.y
    )
  }
}

impl ops::DivAssign for Point {
  fn div_assign(&mut self, other: Point) {
    self.x /= other.x;
    self.y /= other.y;
  }
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "x: {}, y: {}", self.x, self.y)
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
