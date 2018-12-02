use ::ggez::event::{
  Keycode,
  Mod
};

pub struct InputManager {
  keys_down:    Vec<Keycode>,
  keys_up:      Vec<Keycode>,
  keys_pressed: Vec<Keycode>
}

impl InputManager {
  pub fn new() -> Self {
    Self {
      keys_down:    Vec::new(),
      keys_up:      Vec::new(),
      keys_pressed: Vec::new()
    }
  }

  pub fn keys_pressed(&self) -> &Vec<Keycode> {
    &self.keys_pressed
  }

  pub fn keys_down(&self) -> &Vec<Keycode> {
    &self.keys_down
  }

  pub fn keys_up(&self) -> &Vec<Keycode> {
    &self.keys_up
  }

  pub fn key_down(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool) {
    if repeat { return; }
    if !self.keys_pressed.iter().any( |&key| keycode == key ) {
      self.keys_pressed.push(keycode);
    }
    self.keys_down.push(keycode);
  }

  pub fn key_up(&mut self, keycode: Keycode, _keymod: Mod, repeat: bool) {
    if repeat { return (); }
    let index: Option<usize> = self.keys_pressed.iter().position( |&key| keycode == key );
    if let Some(i) = index {
      self.keys_pressed.remove(i);
    }
    self.keys_up.push(keycode);
  }

  pub fn update(&mut self) {
    self.keys_down.clear();
    self.keys_up.clear();
  }
}
