use std::fmt::Debug;

use ::geo::{
  Vector,
  Point,
  num_traits::*,
};
use super::super::Entity;

pub trait Velocity<T>: Entity<T>
where T: Debug + Copy + Num + PartialEq + PartialOrd {
  fn velocity(&self)         -> &Point<T>;
  fn velocity_mut(&mut self) -> &mut Point<T>;
  fn max_velocity(&self)     -> Point<T>;

  fn usable_velocity(&self) -> Point<T> {
    self.velocity().clone()
  }

  fn set_velocity(&mut self, new_velocity: &Point<T>) {
    self.velocity_mut().set(new_velocity);
  }

  fn set_velocity_x(&mut self, val: T) {
    self.velocity_mut().set_x(val);
  }

  fn set_velocity_y(&mut self, val: T) {
    self.velocity_mut().set_y(val);
  }

  fn add_velocity(&mut self, incr_velocity: &Vector<T>) {
    *self.velocity_mut() += incr_velocity;
    if self.velocity().x > self.max_velocity().x {
      let new_x = self.max_velocity().x;
      self.velocity_mut().set_x(new_x);
    } else if self.velocity().x < -self.max_velocity().x {
      let new_x = -self.max_velocity().x;
      self.velocity_mut().set_x(new_x);
    }
    if self.velocity().y > self.max_velocity().y {
      let new_y = self.max_velocity().y;
      self.velocity_mut().set_y(new_y);
    } else if self.velocity().y < -self.max_velocity().y {
      let new_y = -self.max_velocity().y;
      self.velocity_mut().set_y(new_y);
    }
  }

  fn decrease_velocity(&mut self, decr_velocity: &Point<T>) {
    let new_velocity = {
      let velocity = self.velocity();
      let new_x = velocity.x + match velocity.x {
        x if x > 0.0  => -decr_velocity.x,
        x if x < 0.0  =>  decr_velocity.x,
        _             =>  0.0
      };
      let new_y = velocity.y + match velocity.y {
        y if y > 0.0  => -decr_velocity.y,
        y if y < 0.0  =>  decr_velocity.y,
        _             =>  0.0
      };
      Point::new(
        if velocity.x.signum() == new_x.signum() {
          new_x
        } else {
          0.0
        },
        if velocity.y.signum() == new_y.signum() {
          new_y
        } else {
          0.0
        }
      )
    };
    self.velocity_mut().set(&new_velocity);
  }

  fn clear_velocity(&mut self) {
    self.velocity_mut().set(&Point::new(0.0, 0.0));
  }
}
