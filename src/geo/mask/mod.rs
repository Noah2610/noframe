pub mod misc;

use std::fmt::Debug;
use std::ops;

use super::{
  Point,
  Vector,
  Size,
  num_traits::*,
};

use self::misc::{ *, Side::* };

pub trait Mask<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd {
  fn point(&self)         -> &Point<T>;
  fn point_mut(&mut self) -> &mut Point<T>;
  fn size(&self)          -> &Size<T>;
  fn origin(&self)        -> &Origin;

  fn sides_intersect(sides_one: SideCollection<T>, sides_two: SideCollection<T>) -> bool {
    return (
      (
        sides_one.left >= sides_two.left &&
        sides_one.left <  sides_two.right
      ) || (
        sides_one.left  <= sides_two.left &&
        sides_one.right >  sides_two.left
      )
    ) && (
      (
        sides_one.top >= sides_two.top &&
        sides_one.top <  sides_two.bottom
      ) || (
        sides_one.top    <= sides_two.top &&
        sides_one.bottom >  sides_two.top
      )
    );
  }

  fn intersects<M: Mask<T>>(&self, other: &M) -> bool {
    self.is_same(other) || Self::sides_intersect(self.sides(), other.sides())
  }

  fn intersects_round<M: Mask<T>>(&self, other: &M) -> bool where T: Float {
    self.is_same(other) || Self::sides_intersect(self.sides().round(), other.sides().round())
  }

  fn intersects_point(&self, point: &Point<T>) -> bool {
    let sides = self.sides();
    point.x > sides.left && point.x < sides.right &&
      point.y > sides.top && point.y < sides.bottom
  }

  fn is_same<M: Mask<T>>(&self, other: &M) -> bool {
    self.sides() == other.sides()
  }

  fn top_left(&self) -> Point<T> {
    let point = self.point();
    let size  = self.size();
    let two   = T::one() + T::one();

    match self.origin() {
      Origin::TopLeft => Point::new(
        point.x,
        point.y
      ),
      Origin::TopRight => Point::new(
        point.x - size.w,
        point.y
      ),
      Origin::TopCenter => Point::new(
        point.x - size.w / two,
        point.y
      ),
      Origin::BottomLeft => Point::new(
        point.x,
        point.y - size.h
      ),
      Origin::BottomRight => Point::new(
        point.x - size.w,
        point.y - size.h
      ),
      Origin::BottomCenter => Point::new(
        point.x - size.w / two,
        point.y - size.h
      ),
      Origin::CenterLeft => Point::new(
        point.x,
        point.y - size.h / two
      ),
      Origin::CenterRight => Point::new(
        point.x - size.w,
        point.y - size.h / two
      ),
      Origin::Center => Point::new(
        point.x - size.w / two,
        point.y - size.h / two
      )
    }
  }

  fn top_right(&self) -> Point<T> {
    self.top_left() + Vector::new(self.size().w, T::zero())
  }

  fn top_center(&self) -> Point<T> {
    let two = T::one() + T::one();
    self.top_left() + Vector::new(self.size().w / two, T::zero())
  }

  fn bottom_left(&self) -> Point<T> {
    self.top_left() + Vector::new(0.0, self.size().h)
  }

  fn bottom_right(&self) -> Point<T> {
    self.top_left() + Vector::new(self.size().w, self.size().h)
  }

  fn bottom_center(&self) -> Point<T> {
    self.top_left() + Vector::new(self.size().w * 0.5, self.size().h)
  }

  fn center_left(&self) -> Point<T> {
    self.top_left() + Vector::new(0.0, self.size().h * 0.5)
  }

  fn center_right(&self) -> Point<T> {
    self.top_left() + Vector::new(self.size().w, self.size().h * 0.5)
  }

  fn center(&self) -> Point<T> {
    Point::combine(vec![self.point(), &self.size().center()])
  }

  fn side(&self, side: Side) -> T {
    let top_left = self.top_left();
    return match side {
      Top    => top_left.y,
      Bottom => top_left.y + self.size().h,
      Left   => top_left.x,
      Right  => top_left.x + self.size().w
    };
  }

  fn sides(&self) -> SideCollection<T> {
    SideCollection::new(
      self.side(Top),
      self.side(Bottom),
      self.side(Left),
      self.side(Right)
    )
  }
}

#[cfg(test)]
mod tests;
