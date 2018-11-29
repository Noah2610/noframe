use ::geo::{
  NumType,
  point::Point,
  rect::Rect
};
use super::{
  super::Entity,
  velocity::Velocity
};

pub enum Polarity {
  Pos,
  Neg,
  Null
}

impl Polarity {
  pub fn sign(&self) -> i8 {
    match self {
      Polarity::Pos  =>  1,
      Polarity::Neg  => -1,
      Polarity::Null =>  0
    }
  }
}

impl From<i32> for Polarity {
  fn from(num: i32) -> Self {
    match num {
      n if n > 0 => Polarity::Pos,
      n if n < 0 => Polarity::Neg,
      _          => Polarity::Null  // Zero
    }
  }
}

pub enum Step {
  X(Polarity),
  Y(Polarity)
}

impl Step {
  pub fn point(&self) -> Point {
    match self {
      Step::X(pol) => Point::new(pol.sign() as NumType, 0.0),
      Step::Y(pol) => Point::new(0.0, pol.sign() as NumType)
    }
  }
}

enum Axis { X, Y }

impl Axis {
  fn for_each<C: FnMut(Self)>(mut iterate: C) {
    iterate(Axis::X);
    iterate(Axis::Y);
  }

  fn point(&self, value: NumType) -> Point {
    match self {
      Axis::X => Point::new(value, 0.0),
      Axis::Y => Point::new(0.0, value)
    }
  }
}

pub trait Movement: Entity + Velocity {
  /// Moves one pixel in one direction.
  fn step(&mut self, step: Step) {
    self.point_mut().add(&step.point());
  }

  /// This method handles moving the Entity with its current velocity.
  /// The method takes a closure `C`, which determines if the Entity may move to a new position;
  /// it should return a `bool`, `true` if it may move and `false` if not.
  /// The closure is passed a reference to a `Rect` object, which has a `Mask` with the new position.
  ///
  /// There is a problem with this method though:
  /// It modifies the Entity's position (`self`) and therefor needs to use a __mutable reference to self__.
  /// Using a mutable reference means that the passed closure cannot use any reference to self.
  /// Consider the following situation: You have a `GameManager` struct, which has a `player` Entity
  /// and a Vector of some wall Entities `walls`; when the `GameManager` calls the `self.player.move_while`,
  /// it uses a mutable reference to `self` (`GameManager`). Now your passed closure's body might want to
  /// check for collision with the `walls`, but to do that, it must use a reference to them (`self.walls`),
  /// which cannot be done because `self` has already been borrowed mutably by this method.
  /// For this situation there use the method `get_move_while`, which does not directly update the Entity's position,
  /// but rather it _returns the new position_ as a `Point`. Therefor it does not need a mutable reference to self.
  fn move_while<C: Fn(&Rect) -> bool>(&mut self, can_move_to: C) {
    let position = &self.get_move_while(can_move_to);
    self.point_mut().set(&position);
  }

  /// This method almost does exactly the same as `move_while`, except it doesn't update the Entity's position,
  /// but it _returns the new position_ instead. This means it does not need to use a mutable reference to `self`.
  fn get_move_while<C: Fn(&Rect) -> bool>(&self, can_move_to: C) -> Point {
    let mut position = self.point().clone();
    Axis::for_each( |axis| {
      let vel = match axis {
        Axis::X => self.usable_velocity().x,
        Axis::Y => self.usable_velocity().y
      };
      let abs  = vel.abs();
      let sign = if vel as i32 != 0 {
        vel.signum()
      } else { 0.0 };
      let rem  = vel % 1.0;
      // Move by one absolute value at a time
      for _i in 0_i32 ..= abs as i32 {
        let new_position = Point::combine(vec![ &position, &axis.point(sign) ]);
        let new_rect = Rect::new(new_position.clone(), self.size().clone(), self.origin().clone());
        if can_move_to(&new_rect) {
          position = new_position;
        } else { break; }
      }
      // Move by the floating point remainder
      let new_position = Point::combine(vec![ &position, &axis.point(rem) ]);
      let new_rect = Rect::new(new_position.clone(), self.size().clone(), self.origin().clone());
      if can_move_to(&new_rect) {
        position = new_position;
      }
    });
    return position;
  }
}
