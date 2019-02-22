use ggez::event::{
  KeyCode,
  MouseButton,
  KeyMods,
};

pub struct InputManager {
  keys_down:    Vec<KeyCode>,
  keys_up:      Vec<KeyCode>,
  keys_pressed: Vec<KeyCode>,
  mouse_down:   Vec<MouseButton>,
  mouse_up:     Vec<MouseButton>
}

impl InputManager {
  pub fn new() -> Self {
    Self {
      keys_down:    Vec::new(),
      keys_up:      Vec::new(),
      keys_pressed: Vec::new(),
      mouse_down:   Vec::new(),
      mouse_up:     Vec::new()
    }
  }

  pub fn keys_pressed(&self) -> &Vec<KeyCode> {
    &self.keys_pressed
  }

  pub fn keys_down(&self) -> &Vec<KeyCode> {
    &self.keys_down
  }

  pub fn keys_up(&self) -> &Vec<KeyCode> {
    &self.keys_up
  }

  pub fn mouse_down(&self) -> &Vec<MouseButton> {
    &self.mouse_down
  }

  pub fn mouse_up(&self) -> &Vec<MouseButton> {
    &self.mouse_up
  }

  pub fn key_down(
    &mut self,
    keycode:  KeyCode,
    _keymods: KeyMods,
    repeat:   bool
  ) {
    if repeat { return; }
    if !self.keys_pressed.iter().any( |&key| keycode == key ) {
      self.keys_pressed.push(keycode);
    }
    self.keys_down.push(keycode);
  }

  pub fn key_up(
    &mut self,
    keycode:  KeyCode,
    _keymods: KeyMods,
    repeat:   bool
  ) {
    if repeat { return (); }
    let index: Option<usize> = self.keys_pressed.iter().position( |&key| keycode == key );
    if let Some(i) = index {
      self.keys_pressed.remove(i);
    }
    self.keys_up.push(keycode);
  }

  pub fn add_mouse_down(&mut self, mouse_button: MouseButton, x: i32, y: i32) {
    self.mouse_down.push(mouse_button);
  }

  pub fn add_mouse_up(&mut self, mouse_button: MouseButton, x: i32, y: i32) {
    self.mouse_up.push(mouse_button);
  }

  pub fn update(&mut self) {
    self.keys_down.clear();
    self.keys_up.clear();
    self.mouse_down.clear();
    self.mouse_up.clear();
  }
}
