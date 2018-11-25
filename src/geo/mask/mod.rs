use super::NumType;
use super::{
  point::Point,
  size::Size
};

pub mod misc;

use self::misc::{ *, Side::* };

pub trait Mask {
  fn point(&self)         -> &Point;
  fn point_mut(&mut self) -> &mut Point;
  fn size(&self)          -> &Size;
  fn origin(&self)        -> &Origin;

  fn rectangle(&self) -> ::ggez::graphics::Rect {
    let top_left: Point = self.top_left();
    let size:     &Size = self.size();
    return [
      top_left.x, top_left.y,
      size.w,     size.h
    ].into();
  }

  fn intersects<M: Mask>(&self, other: &M) -> bool {
    let self_sides:  SideCollection = self.sides();
    let other_sides: SideCollection = other.sides();
    return self.is_same(other) || (
      (
        (
          self_sides.left > other_sides.left &&
          self_sides.left < other_sides.right
        ) || (
          self_sides.left  < other_sides.left &&
          self_sides.right > other_sides.left
        )
      ) && (
        (
          self_sides.top > other_sides.top &&
          self_sides.top < other_sides.bottom
        ) || (
          self_sides.top    < other_sides.top &&
          self_sides.bottom > other_sides.top
        )
      )
    );
  }

  fn is_same<M: Mask>(&self, other: &M) -> bool {
    self.sides() == other.sides()
  }

  fn top_left(&self) -> Point {
    let point: &Point = self.point();
    let size:  &Size  = self.size();

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
        point.x - size.w / 2.0,
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
        point.x - size.w / 2.0,
        point.y - size.h
      ),
      Origin::CenterLeft => Point::new(
        point.x,
        point.y - size.h / 2.0
      ),
      Origin::CenterRight => Point::new(
        point.x - size.w,
        point.y - size.h / 2.0
      ),
      Origin::Center => Point::new(
        point.x - size.w / 2.0,
        point.y - size.h / 2.0
      )
    }
  }

  fn center(&self) -> Point {
    Point::combine(vec![self.point(), &self.size().center()])
  }

  fn side(&self, side: Side) -> NumType {
    let top_left: Point = self.top_left();
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
