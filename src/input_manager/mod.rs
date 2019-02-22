use ggez::event::{
  KeyCode,
  MouseButton,
  KeyMods,
};

use crate::geo::{ GNum, Vector, Point };

/// This struct is used by `InputManager` for mouse buttons.
/// It groups the ggez `MouseButton` enum together with the mouse's position `Point`.
#[derive(Clone, Copy, PartialEq)]
pub struct MouseButtonData {
  pub button: MouseButton,
  pub point:  Point,
}

impl MouseButtonData {
  pub fn new(button: MouseButton, x: f32, y: f32) -> Self {
    Self {
      button,
      point: Point::new(x as GNum, y as GNum),
    }
  }
}

/// This struct is supposed to make input handling easier and quicker.
/// To get it to work with your ggez MainState, you will need to manually
/// call the following methods on the `InputManager`, from the proper `EventHandler` methods:
///   ```
///   # extern crate ggez;
///   # extern crate noframe;
///   # use ggez::event::{ EventHandler, KeyCode, KeyMods, MouseButton };
///   # use ggez::{ Context, GameResult };
///   # use noframe::input_manager::InputManager;
///   struct MainState(InputManager);
///
///   impl EventHandler for MainState {
///     # fn update(&mut self, _ctx: &mut Context) -> GameResult { Ok(()) }
///     # fn draw(&mut self, _ctx: &mut Context)   -> GameResult { Ok(()) }
///     fn key_down_event(
///       &mut self,
///       ctx:     &mut Context,
///       keycode: KeyCode,
///       keymods: KeyMods,
///       repeat:  bool
///     ) {
///       self.0.set_key_down(keycode, keymods, repeat);
///     }
///
///     fn key_up_event(
///       &mut self,
///       _ctx:    &mut Context,
///       keycode: KeyCode,
///       keymods: KeyMods
///     ) {
///       self.0.set_key_up(keycode, keymods);
///     }
///
///     fn mouse_button_down_event(
///       &mut self,
///       _ctx:    &mut Context,
///       button:  MouseButton,
///       x:       f32,
///       y:       f32,
///     ) {
///       self.0.set_mouse_down(button, x, y);
///     }
///
///     fn mouse_button_up_event(
///       &mut self,
///       _ctx:    &mut Context,
///       button:  MouseButton,
///       x:       f32,
///       y:       f32,
///     ) {
///       self.0.set_mouse_up(button, x, y);
///     }
///
///     fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
///       self.0.set_mouse_wheel(x, y);
///     }
///
///     fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
///       self.0.set_mouse_motion(x, y, dx, dy);
///     }
///   }
///   ```
pub struct InputManager {
  keys_down:     Vec<KeyCode>,
  keys_up:       Vec<KeyCode>,
  keys_pressed:  Vec<KeyCode>,
  mouse_down:    Vec<MouseButtonData>,
  mouse_up:      Vec<MouseButtonData>,
  mouse_pressed: Vec<MouseButton>,
  mouse_scroll:  i32,
  mouse_point:   Point,
  mouse_motion:  Vector,
}

impl InputManager {
  /// Creates and returns a new `InputManager`.
  pub fn new() -> Self {
    Self {
      keys_down:     Vec::new(),
      keys_up:       Vec::new(),
      keys_pressed:  Vec::new(),
      mouse_down:    Vec::new(),
      mouse_up:      Vec::new(),
      mouse_pressed: Vec::new(),
      mouse_scroll:  0,
      mouse_point:   Point::new(GNum::from(0i8), GNum::from(0i8)),
      mouse_motion:  Vector::new(GNum::from(0i8), GNum::from(0i8)),
    }
  }

  /// Returns a reference to a `Vec` containing all keyboard `KeyCode`s,
  /// which were pushed _down_ in this frame.
  pub fn keys_down(&self) -> &Vec<KeyCode> {
    &self.keys_down
  }

  /// Returns a reference to a `Vec` containing all keyboard `KeyCode`s,
  /// which were _released_ in this frame.
  pub fn keys_up(&self) -> &Vec<KeyCode> {
    &self.keys_up
  }

  /// Returns a reference to a `Vec` containing all keyboard `KeyCode`s,
  /// which are currently being _pressed_ down (aka were pushed down but haven't been released yet).
  pub fn keys_pressed(&self) -> &Vec<KeyCode> {
    &self.keys_pressed
  }

  /// Returns `true` if the user pressed down the `key` KeyCode in this frame.
  pub fn has_key_down(&self, key: KeyCode) -> bool {
    self.keys_down.iter().any( |&down| down == key )
  }

  /// Returns `true` if the user released the `key` KeyCode in this frame.
  pub fn has_key_up(&self, key: KeyCode) -> bool {
    self.keys_up.iter().any( |&up| up == key )
  }

  /// Returns `true` if the `key` KeyCode is currently pressed down.
  pub fn is_key_pressed(&self, key: KeyCode) -> bool {
    self.keys_pressed.iter().any( |&pressed| pressed == key )
  }

  /// Returns a reference to a `Vec` containing all mouse buttons (including their position in the window)
  /// which were pushed _down_ in this frame.
  pub fn mouse_down(&self) -> &Vec<MouseButtonData> {
    &self.mouse_down
  }

  /// Returns a reference to a `Vec` containing all mouse buttons (including their position in the window)
  /// which were _released_ in this frame.
  pub fn mouse_up(&self) -> &Vec<MouseButtonData> {
    &self.mouse_up
  }

  /// Returns a reference to a `Vec` containing all mouse buttons (including their position in the window)
  /// which are currently being _pressed_ down (aka were pushed down but haven't been released yet).
  pub fn mouse_pressed(&self) -> &Vec<MouseButton> {
    &self.mouse_pressed
  }

  /// Returns `true` if the user pressed down the `button` MouseButton in this frame.
  pub fn has_mouse_down(&self, button: MouseButton) -> bool {
    self.mouse_down.iter().any( |down| down.button == button )
  }

  /// Returns `true` if the user released the `button` MouseButton in this frame.
  pub fn has_mouse_up(&self, button: MouseButton) -> bool {
    self.mouse_up.iter().any( |up| up.button == button )
  }

  /// Returns `true` if the `button` MouseButton is currently pressed down.
  pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
    self.mouse_pressed.iter().any( |&pressed| pressed == button )
  }

  /// Returns an `i32` representing the direction (and "distance")
  /// scrolled vertically with the mouse wheel.
  /// A __positive__ number means it was scrolled __up__ / __away__ from the user,
  /// a __negative__ number means it was scrolled __down__ / __towards__ the user.
  pub fn mouse_scroll(&self) -> i32 {
    self.mouse_scroll
  }

  /// Returns the current mouse position in the window as a `&Point`.
  pub fn mouse_point(&self) -> &Point {
    &self.mouse_point
  }

  /// Returns the mouse's movement relative to the previous frame as a `&Vector`.
  pub fn mouse_motion(&self) -> &Vector {
    &self.mouse_motion
  }

  /// Call this method from your ggez MainState's `key_down_event` method.
  pub fn set_key_down(
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

  /// Call this method from your ggez MainState's `key_up_event` method.
  pub fn set_key_up(
    &mut self,
    keycode:  KeyCode,
    _keymods: KeyMods
  ) {
    let index = self.keys_pressed.iter().position( |&key| keycode == key );
    if let Some(i) = index {
      self.keys_pressed.remove(i);
    }
    self.keys_up.push(keycode);
  }

  /// Call this method from your ggez MainState's `mouse_button_down_event` method.
  pub fn set_mouse_down(&mut self, mouse_button: MouseButton, x: f32, y: f32) {
    let data = MouseButtonData::new(mouse_button, x, y);
    if !self.mouse_pressed.iter().any( |&d| d == mouse_button ) {
      self.mouse_pressed.push(mouse_button);
    }
    self.mouse_down.push(data);
  }

  /// Call this method from your ggez MainState's `mouse_button_up_event` method.
  pub fn set_mouse_up(&mut self, mouse_button: MouseButton, x: f32, y: f32) {
    let data = MouseButtonData::new(mouse_button, x, y);
    let index = self.mouse_pressed.iter().position( |&d| d == mouse_button );
    if let Some(i) = index {
      self.mouse_pressed.remove(i);
    }
    self.mouse_up.push(data);
  }

  /// Call this method from your ggez MainState's `mouse_wheel_event` method.
  pub fn set_mouse_wheel(&mut self, _x: f32, y: f32) {
    self.mouse_scroll += y as i32;
  }

  /// Call this method from your ggez MainState's `mouse_motion_event` method.
  pub fn set_mouse_motion(&mut self, x: f32, y: f32, dx: f32, dy: f32) {
    self.mouse_point   = Point::new(GNum::from(x),   GNum::from(y));
    self.mouse_motion += Vector::new(GNum::from(dx), GNum::from(dy));
  }

  /// Call this method from your ggez MainState's `update` method every tick.
  pub fn update(&mut self) {
    self.keys_down.clear();
    self.keys_up.clear();
    self.mouse_down.clear();
    self.mouse_up.clear();
    self.mouse_scroll = 0;
    self.mouse_motion = Vector::new(GNum::from(0i8), GNum::from(0i8));
  }
}

#[cfg(test)]
mod tests;
