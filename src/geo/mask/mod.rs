pub mod misc;

use super::{
  GNum,
  Point,
  Vector,
  Size,
};

use self::misc::{ *, Side::* };

pub trait Mask {
  fn point(&self)         -> &Point;
  fn point_mut(&mut self) -> &mut Point;
  fn size(&self)          -> &Size;
  fn origin(&self)        -> &Origin;

  fn sides_intersect(sides_one: SideCollection, sides_two: SideCollection) -> bool {
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

  fn intersects<M: Mask>(&self, other: &M) -> bool {
    self.is_same(other) || Self::sides_intersect(self.sides(), other.sides())
  }

  fn intersects_round<M: Mask>(&self, other: &M) -> bool {
    self.is_same(other) || Self::sides_intersect(self.sides().round(), other.sides().round())
  }

  fn intersects_point(&self, point: &Point) -> bool {
    let sides = self.sides();
    point.x > sides.left && point.x < sides.right &&
      point.y > sides.top && point.y < sides.bottom
  }

  fn is_same<M: Mask>(&self, other: &M) -> bool {
    self.sides() == other.sides()
  }

  fn top_left(&self) -> Point {
    let point = self.point();
    let size  = self.size();
    let two   = GNum::from(2i8);

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

  fn top_right(&self) -> Point {
    self.top_left() + Vector::new(self.size().w, GNum::from(0i8))
  }

  fn top_center(&self) -> Point {
    self.top_left() + Vector::new(self.size().w / GNum::from(2i8), GNum::from(0i8))
  }

  fn bottom_left(&self) -> Point {
    self.top_left() + Vector::new(0.0, self.size().h)
  }

  fn bottom_right(&self) -> Point {
    self.top_left() + Vector::new(self.size().w, self.size().h)
  }

  fn bottom_center(&self) -> Point {
    self.top_left() + Vector::new(self.size().w * 0.5, self.size().h)
  }

  fn center_left(&self) -> Point {
    self.top_left() + Vector::new(0.0, self.size().h * 0.5)
  }

  fn center_right(&self) -> Point {
    self.top_left() + Vector::new(self.size().w, self.size().h * 0.5)
  }

  fn center(&self) -> Point {
    self.point() + self.size().center()
  }

  fn side(&self, side: Side) -> GNum {
    let top_left = self.top_left();
    return match side {
      Top    => top_left.y,
      Bottom => top_left.y + self.size().h,
      Left   => top_left.x,
      Right  => top_left.x + self.size().w
    };
  }

  fn sides(&self) -> SideCollection {
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
