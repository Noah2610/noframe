use std::time::{
  Instant,
  Duration
};
use std::fmt;

#[derive(Clone)]
pub struct Deltatime {
  value:       Duration,
  last_update: Instant
}

impl Deltatime {
  pub fn new() -> Self {
    Self {
      value:       Duration::new(0, 0),
      last_update: Instant::now()
    }
  }

  /// Returns the current deltatime value as a `Duration`.
  pub fn get(&self) -> Duration {
    self.value
  }

  /// Returns the current deltatime value in seconds.
  pub fn secs(&self) -> f32 {
    self.value.as_secs() as f32 + self.value.subsec_millis() as f32 * 0.001
  }

  /// Returns the current deltatime value in milliseconds.
  pub fn millis(&self) -> f32 {
    self.secs() * 1000.0
  }

  /// Resets the current deltatime value; sets the value to a `Duration` of `0`.
  pub fn reset(&mut self) {
    self.value       = Duration::new(0, 0);
    self.last_update = Instant::now();
  }

  /// This method should be called every tick.
  /// It updates the deltatime `Duration` value that is returned by the `get` method.
  pub fn update(&mut self) {
    self.value = self.last_update.elapsed();
    self.last_update = Instant::now();
  }
}

impl fmt::Display for Deltatime {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}.{}", self.value.as_secs(), self.value.subsec_millis())
  }
}
