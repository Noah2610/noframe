use ::geo::{
  GNum,
  Vector,
  Point,
  Rect,
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
  pub fn vector(&self) -> Vector {
    let zero = GNum::from(0i8);
    match self {
      Step::X(pol) => Vector::new(pol.sign().into(), zero),
      Step::Y(pol) => Vector::new(zero, pol.sign().into())
    }
  }
}

enum Axis { X, Y }

impl Axis {
  fn for_each<C: FnMut(Self)>(mut iterate: C) {
    iterate(Axis::X);
    iterate(Axis::Y);
  }

  fn vector(&self, value: GNum) -> Vector {
    let zero = GNum::from(0i8);
    match self {
      Axis::X => Vector::new(value, zero),
      Axis::Y => Vector::new(zero, value)
    }
  }
}

pub trait Movement: Entity + Velocity {
  /// Moves one pixel in one direction.
  fn step(&mut self, step: Step) {
    *self.point_mut() += step.vector();
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
    let position = self.get_move_while(can_move_to);
    *self.point_mut() = position;
  }

  /// This method almost does exactly the same as `move_while`, except it doesn't update the Entity's position,
  /// but it _returns the new position_ instead. This means it does not need to use a mutable reference to `self`.
  fn get_move_while<C: Fn(&Rect) -> bool>(&self, can_move_to: C) -> Point {
    let zero = GNum::from(0i8);
    let one  = GNum::from(1i8);
    let mut position = self.point().clone();
    Axis::for_each( |axis| {
      let vel = match axis {
        Axis::X => self.usable_velocity().x,
        Axis::Y => self.usable_velocity().y
      };
      let abs  = vel.abs() as usize;
      let sign = if vel != zero {
        vel.signum()
      } else { zero };
      let rem = vel % one;
      // Move by one absolute value at a time
      for _ in 0_usize ..= abs {
        let new_position = position + axis.vector(sign);
        let new_rect = Rect::new(new_position.clone(), self.size().clone(), self.origin().clone());
        if can_move_to(&new_rect) {
          position = new_position;
        } else { break; }
      }
      // Move by the floating point remainder
      let new_position = position + axis.vector(rem);
      let new_rect = Rect::new(new_position.clone(), self.size().clone(), self.origin().clone());
      if can_move_to(&new_rect) {
        position = new_position;
      }
    });
    return position;
  }
}
