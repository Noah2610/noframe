use std::{ ops, fmt, convert };
use super::NumType;
use super::num_traits::*;

// Experimental trait aliases: https://github.com/rust-lang/rust/issues/41517
//trait PointBounds = Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast;

#[derive(Debug, Clone, PartialEq)]
pub struct Point<T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast> {
  pub x: T,
  pub y: T,
}

impl<T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast> Point<T> {
  /// Returns a new `Point` with the passed `x` and `y` values.
  /// # Example
  ///   ```
  ///   use noframe::geo::point::Point;
  ///
  ///   let point = Point::new(32.0, 64.0);
  ///
  ///   assert_eq!((32.0, 64.0), (point.x, point.y))
  ///   ```
  pub fn new(x: T, y: T) -> Self {
    Self { x, y }
  }

  // pub fn zero() -> Self {
  //   Self {
  //     x: 0 as T,
  //     y: 0 as T,
  //   }
  // }

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
  pub fn combine(points: Vec<&Point<T>>) -> Point<T> {
    let mut point_acc: Point<T> = Point::new(T::from(0).unwrap(), T::from(0).unwrap());
    for point in points {
      point_acc.add(point);
    }
    point_acc
  }

  /// Set the `x` and `y` values to the values of the passed `Point` reference.
  pub fn set(&mut self, point: &Point<T>) {
    self.x = point.x;
    self.y = point.y;
  }

  /// Set the `x` value.
  pub fn set_x(&mut self, value: T) {
    self.x = value;
  }

  /// Set the `y` value.
  pub fn set_y(&mut self, value: T) {
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
  pub fn add(&mut self, point: &Point<T>) {
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
  pub fn as_tup(&self) -> (T, T) {
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
  pub fn inverted(&self) -> Point<T> where T: Signed {
    Point::new(self.x.neg(), self.y.neg())
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
  pub fn invert(&mut self) where T: Signed {
    self.x = self.x.neg();
    self.y = self.y.neg();
    // self.x *= -1.0;
    // self.y *= -1.0;
  }

  pub fn mult_axes_by(&self, mult: T) -> Point<T> {
    Point::new(
      self.x * mult,
      self.y * mult
    )
  }

  /// Round the Point's `x` and `y` values.
  pub fn round(&mut self) where T: Float {
    self.x = self.x.round();
    self.y = self.y.round();
  }
}

impl<T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast> ops::Add for Point<T> {
  type Output = Point<T>;
  fn add(self, other: Point<T>) -> Point<T> {
    Point::combine(vec![&self, &other])
  }
}

impl<T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast> ops::AddAssign for Point<T> {
  fn add_assign(&mut self, other: Point<T>) {
    self.x += other.x;
    self.y += other.y;
  }
}

impl<T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast> ops::Sub for Point<T> {
  type Output = Point<T>;
  fn sub(self, other: Point<T>) -> Point<T> {
    Point::new(
      self.x - other.x,
      self.y - other.y
    )
  }
}

impl<T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast> ops::SubAssign for Point<T> {
  fn sub_assign(&mut self, other: Point<T>) {
    self.x -= other.x;
    self.y -= other.y;
  }
}

impl<T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast> ops::Mul for Point<T> {
  type Output = Point<T>;
  fn mul(self, other: Point<T>) -> Point<T> {
    Point::new(
      self.x * other.x,
      self.y * other.y
    )
  }
}

impl<T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast> ops::MulAssign for Point<T> {
  fn mul_assign(&mut self, other: Point<T>) {
    self.x *= other.x;
    self.y *= other.y;
  }
}

impl<T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast> ops::Div for Point<T> {
  type Output = Point<T>;
  fn div(self, other: Point<T>) -> Point<T> {
    Point::new(
      self.x / other.x,
      self.y / other.y
    )
  }
}

impl<T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast> ops::DivAssign for Point<T> {
  fn div_assign(&mut self, other: Point<T>) {
    self.x /= other.x;
    self.y /= other.y;
  }
}

impl<T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast + fmt::Display> fmt::Display for Point<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "x: {}, y: {}", self.x, self.y)
  }
}

/// Implement the `From` trait for ggez's `Point2` struct,
/// so that ggez's `Point2` struct can easily be created from our `Point` struct.
impl<'a, T: Copy + Into<NumType> + Num + NumOps + NumAssignOps + ToPrimitive + FromPrimitive + NumCast> From<&'a Point<T>> for ::ggez::nalgebra::Point2<NumType> {
  fn from(point: &Point<T>) -> Self {
    Self::new(
      point.x.into(),
      point.y.into()
    )
  }
}

#[cfg(test)]
mod tests;
