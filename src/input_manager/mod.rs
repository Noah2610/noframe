use ::ggez::event::{
  Keycode,
  Mod
};

pub struct InputManager {
  pressed_keys: Vec<Keycode>
}

impl InputManager {
  pub fn new() -> Self {
    Self {
      pressed_keys: Vec::new()
    }
  }

  pub fn keys(&self) -> &Vec<Keycode> {
    &self.pressed_keys
  }

  pub fn key_down(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool) {
    if repeat { return; }
    if !self.pressed_keys.iter().any( |&key| keycode == key ) {
      self.pressed_keys.push(keycode);
    }
  }

  pub fn key_up(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool) {
    if repeat { return (); }
    let index: Option<usize> = self.pressed_keys.iter().position( |&key| keycode == key );
    if let Some(i) = index {
      self.pressed_keys.remove(i);
    }
  }
}
