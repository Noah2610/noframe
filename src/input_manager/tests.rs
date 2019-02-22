use ggez::event::{
  KeyCode,
  MouseButton,
  KeyMods,
};

use crate::geo::{ Vector, Point };
use super::{
  InputManager,
  MouseButtonData as MBD,
};

fn get_keys() -> (InputManager, Vec<KeyCode>, Vec<KeyCode>, KeyMods) {
  (
    InputManager::new(),
    vec![ KeyCode::A, KeyCode::B, ],
    vec![ KeyCode::X, KeyCode::Y, ],
    KeyMods::empty()
  )
}

fn get_mouse_btns() -> (InputManager, Vec<MBD>, Vec<MBD>) {
  (
    InputManager::new(),
    vec![ MBD::new(MouseButton::Left, 10.0, 20.0), MBD::new(MouseButton::Right, 100.0, 200.0) ],
    vec![ MBD::new(MouseButton::Middle, 0.0, 0.0) ],
    )
}

#[test]
fn test_keys_down() {
  let (mut manager, keys, nokeys, mods) = get_keys();

  for key in keys.iter() {      //   vvvvv  repeat
    manager.set_key_down(*key, mods, false);
  }
  for nokey in nokeys.iter() {  //     vvvv repeat
    manager.set_key_down(*nokey, mods, true);
  }

  {
    let keys_down = manager.keys_down();
    for key in keys.iter() {
      assert!(keys_down.contains(key), "Should contain key");
    }
    for nokey in nokeys.iter() {
      assert!(!keys_down.contains(nokey), "Shouldn't contain key");
    }
  }

  manager.update();

  let keys_down = manager.keys_down();
  for key in keys.iter() {
    assert!(!keys_down.contains(key), "Shouldn't contain key after update");
  }
}

#[test]
fn test_keys_up() {
  let (mut manager, keys, nokeys, mods) = get_keys();

  for key in keys.iter() {
    manager.set_key_up(*key, mods);
  }

  {
    let keys_up = manager.keys_up();
    for key in keys.iter() {
      assert!(keys_up.contains(key), "Should contain key");
    }
    for nokey in nokeys.iter() {
      assert!(!keys_up.contains(nokey), "Shouldn't contain key");
    }
  }

  manager.update();

  let keys_up = manager.keys_up();
  for key in keys.iter() {
    assert!(!keys_up.contains(key), "Shouldn't contain key after update");
  }
}

#[test]
fn test_has_key_down() {
  let (mut manager, keys, nokeys, mods) = get_keys();

  for key in keys.iter() {      //   vvvvv  repeat
    manager.set_key_down(*key, mods, false);
  }
  for nokey in nokeys.iter() {  //     vvvv repeat
    manager.set_key_down(*nokey, mods, true);
  }

  for key in keys.iter() {
    assert!(manager.has_key_down(key), "Key should be down (`has_key_down`)");
  }
  for nokey in nokeys.iter() {
    assert!(!manager.has_key_down(nokey), "Key shouldn't be down (`has_key_down`)")
  }

  manager.update();

  for key in keys.iter() {
    assert!(!manager.has_key_down(key), "Key shouldn't be down after update (`has_key_down`)");
  }
}

#[test]
fn test_has_key_up() {
  let (mut manager, keys, nokeys, mods) = get_keys();

  for key in keys.iter() {
    manager.set_key_up(*key, mods);
  }

  for key in keys.iter() {
    assert!(manager.has_key_up(key), "Key should be up (`has_key_up`)");
  }
  for nokey in nokeys.iter() {
    assert!(!manager.has_key_up(nokey), "Key shouldn't be up (`has_key_up`)")
  }

  manager.update();

  for key in keys.iter() {
    assert!(!manager.has_key_up(key), "Key shouldn't be up after update (`has_key_up`)");
  }
}

#[test]
fn test_is_key_pressed() {
  let (mut manager, keys, nokeys, mods) = get_keys();

  for key in keys.iter() {      //   vvvvv  repeat
    manager.set_key_down(*key, mods, false);
  }
  for nokey in nokeys.iter() {  //     vvvv repeat
    manager.set_key_down(*nokey, mods, true);
  }

  manager.update();

  for key in keys.iter() {
    assert!(manager.is_key_pressed(key), "Key should be pressed (`is_key_pressed`)");
  }
  for nokey in nokeys.iter() {
    assert!(!manager.is_key_pressed(key), "Key shouldn't be pressed (`is_key_pressed`)");
  }

  for key in keys.iter() {
    manager.set_key_up(*key, mods);
  }

  for key in keys.iter() {
    assert!(!manager.is_key_pressed(key), "Key shouldn't be pressed after release (`is_key_pressed`)");
  }
}

#[test]
fn test_mouse_down() {
  let (mut manager, btns, nobtns) = get_mouse_btns();

  for btn in btns.iter() {
    manager.set_mouse_down(btn.button, btn.point.x, btn.point.y);
  }

  {
    let btns_down = manager.mouse_down();
    for btn in btns.iter() {
      assert!(btns_down.contains(btn), "Should contain mouse button");
    }
    for nobtn in nobtns.iter() {
      assert!(!btns_down.contains(nobtn), "Shouldn't contain mouse button");
    }
  }

  manager.update();

  let btns_down = manager.mouse_down();
  for btn in btns.iter() {
    assert!(!btns_down.contains(btn), "Shouldn't contain button after update");
  }
}

#[test]
fn test_mouse_up() {
  let (mut manager, btns, nobtns) = get_mouse_btns();

  for btn in btns.iter() {
    manager.set_mouse_up(btn.button, btn.point.x, btn.point.y);
  }

  {
    let btns_up = manager.mouse_up();
    for btn in btns.iter() {
      assert!(btns_up.contains(btn), "Should contain mouse button");
    }
    for nobtn in nobtns.iter() {
      assert!(!btns_up.contains(nobtn), "Shouldn't contain mouse button");
    }
  }

  manager.update();

  let btns_up = manager.mouse_up();
  for btn in btns.iter() {
    assert!(!btns_up.contains(btn), "Shouldn't contain button after update");
  }
}

#[test]
fn test_has_mouse_down() {
  let (mut manager, btns, nobtns) = get_mouse_btns();

  for btn in btns.iter() {
    manager.set_mouse_down(btn.button, btn.point.x, btn.point.y);
  }

  for btn in btns.iter() {
    assert!(manager.has_mouse_down(btn.button), "Mouse button should be down (`has_mouse_down`)");
  }
  for nobtn in nobtns.iter() {
    assert!(!manager.has_mouse_down(nobtn.button), "Mouse button should not be down (`has_mouse_down`)"),
  }

  manager.update();

  for btn in btns.iter() {
    assert!(!manager.has_mouse_down(btn.button), "Mouse button should not be down after update (`has_mouse_down`)");
  }
}

#[test]
fn test_has_mouse_up() {
  let (mut manager, btns, nobtns) = get_mouse_btns();

  for btn in btns.iter() {
    manager.set_mouse_up(btn.button, btn.point.x, btn.point.y);
  }

  for btn in btns.iter() {
    assert!(manager.has_mouse_up(btn.button), "Mouse button should be down (`has_mouse_up`)");
  }
  for nobtn in nobtns.iter() {
    assert!(!manager.has_mouse_up(nobtn.button), "Mouse button should not be down (`has_mouse_up`)"),
  }

  manager.update();

  for btn in btns.iter() {
    assert!(!manager.has_mouse_up(btn.button), "Mouse button should not be down after update (`has_mouse_up`)");
  }
}

#[test]
fn test_mouse_wheel() {
  let mut manager = InputManager::new();

  let ints = vec![-1, -2, 1, 1, 2, 2, 3, 3];
  for i in ints.iter() {
    manager.set_mouse_wheel(0.0, *i as f32);
  }

  let sum: i32 = ints.iter().sum();
  assert_eq!(sum, manager.mouse_scroll(), "Should have proper scroll value");

  manager.update();

  assert_eq!(0, manager.mouse_scroll(), "Should have reset scroll value to 0");
}

#[test]
fn test_mouse_motion() {
  let mut manager = InputManager::new();

  let positions = vec![
    [ 0.0,  0.0,  0.0, 0.0 ],
    [ 10.0, 20.0, 0.0, 0.0 ],
    [ 20.0, 20.0, 5.0, 2.0 ],
    [ 30.0, 40.0, 1.5, 3.6 ],
    [ 60.0, 80.0, 0.2, 4.9 ],
  ];

  for data in positions.iter() {
    manager.set_mouse_motion(data[0], data[1], data[2], data[3]);
    assert_eq!(&Point::new( data[0], data[1]), manager.mouse_point(),  "Mouse should be in correct position");
    assert_eq!(&Vector::new(data[2], data[3]), manager.mouse_motion(), "Mouse's relative motion should be correct");
    manager.update();
  }

  manager.update();
  let last = positions.last().unwrap();
  assert_eq!(&Point::new(last[0], last[1]), manager.mouse_point(),  "Last mouse point should be preserved between updates");
  assert_eq!(&Vector::new(0.0, 0.0),        manager.mouse_motion(), "Mouse motion should be reset between updates");
}
