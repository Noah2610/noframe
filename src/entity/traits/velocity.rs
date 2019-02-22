use ::geo::{
  GNum,
  Vector,
};
use super::super::Entity;

  pub trait Velocity: Entity {
  fn velocity(&self)         -> &Vector;
  fn velocity_mut(&mut self) -> &mut Vector;
  fn max_velocity(&self)     -> Vector;

  fn usable_velocity(&self) -> Vector {
    self.velocity().clone()
  }

  fn set_velocity(&mut self, new_velocity: &Vector) {
    self.velocity_mut().x = new_velocity.x;
    self.velocity_mut().y = new_velocity.y;
  }

  fn set_velocity_x(&mut self, val: GNum) {
    self.velocity_mut().x = val;
  }

  fn set_velocity_y(&mut self, val: GNum) {
    self.velocity_mut().y = val;
  }

  fn add_velocity(&mut self, incr_velocity: &Vector) {
    *self.velocity_mut() += incr_velocity;
    if self.velocity().x > self.max_velocity().x {
      let new_x = self.max_velocity().x;
      self.velocity_mut().x = new_x;
    } else if self.velocity().x < -self.max_velocity().x {
      let new_x = -self.max_velocity().x;
      self.velocity_mut().x = new_x;
    }
    if self.velocity().y > self.max_velocity().y {
      let new_y = self.max_velocity().y;
      self.velocity_mut().y = new_y;
    } else if self.velocity().y < -self.max_velocity().y {
      let new_y = -self.max_velocity().y;
      self.velocity_mut().y = new_y;
    }
  }

  fn decrease_velocity(&mut self, decr_velocity: &Vector) {
    let new_velocity = {
      let zero = GNum::from(0i8);
      let velocity = self.velocity();
      let new_x = velocity.x + match velocity.x {
        x if x > zero => -decr_velocity.x,
        x if x < zero =>  decr_velocity.x,
        _             =>  zero
      };
      let new_y = velocity.y + match velocity.y {
        y if y > zero => -decr_velocity.y,
        y if y < zero =>  decr_velocity.y,
        _             =>  zero
      };
      Vector::new(
        if velocity.x.signum() == new_x.signum() {
          new_x
        } else {
          zero
        },
        if velocity.y.signum() == new_y.signum() {
          new_y
        } else {
          zero
        }
      )
    };
    self.set_velocity(&new_velocity);
  }

  fn clear_velocity(&mut self) {
    let zero = GNum::from(0i8);
    self.set_velocity(&Vector::new(zero, zero));
  }
}
