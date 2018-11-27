use ::geo::{
  NumType,
  point::Point,
  mask::Mask,
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

pub enum Axis { X, Y }

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

  fn move_while<C: Fn(&Rect) -> bool>(&mut self, can_move_to: C) {
    Axis::for_each( |axis| {
      let pos = match axis {
        Axis::X => self.velocity().x,
        Axis::Y => self.velocity().y,
      };
      let abs  = pos.abs();
      let sign = if pos as i32 != 0 {
        pos.signum()
      } else { 0.0 };
      let rem  = pos % 1.0;
      // Move by one absolute value at a time
      for _i in 0_i32 ..= abs as i32 {
        let new_position = Point::combine(vec![ self.point(), &axis.point(sign) ]);
        let new_rect = Rect::new(new_position, self.size().clone(), self.origin().clone());
        if can_move_to(&new_rect) {
          self.point_mut().set(&new_rect.point());
        } else { break; }
      }
      // Move by the floating point remainder
      let new_position = Point::combine(vec![ self.point(), &axis.point(rem) ]);
      let new_rect = Rect::new(new_position, self.size().clone(), self.origin().clone());
      if can_move_to(&new_rect) {
        self.point_mut().set(&new_rect.point());
      }
    });

    // let x      = self.point().x;
    // let x_abs  = x.abs();
    // let sign_x = x.signum();
    // let rem_x  = x % 1.0;

    // for _i in 0_i32 ..= x_abs as i32 {
    //   let new_position = Point::combine(vec![ self.point(), &Point::new(sign_x, 0.0) ]);
    //   if can_move_to(&new_position) {
    //     self.point_mut().set(&new_position);
    //   } else { break; }
    // }
    // // Move by the floating point remainder
    // let new_position = Point::combine(vec![ self.point(), &Point::new(rem_x, 0.0) ]);
    // if can_move_to(&new_position) {
    //   self.point_mut().set(&new_position);
    // }

    // let y      = self.point().y;
    // let y_abs  = y.abs();
    // let sign_y = y.signum();
    // let rem_y  = y_abs % 1.0;
    // for _i in 0_i32 ..= y_abs as i32 {
    //   let new_position = Point::combine(vec![ self.point(), &Point::new(0.0, sign_y) ]);
    //   if can_move_to(&new_position) {
    //     self.point_mut().set(&new_position);
    //   } else { break; }
    // }
  }
}
