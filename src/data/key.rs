use windows::System::VirtualKey;

// Key action
#[derive(Clone, Copy, PartialEq)]
pub enum KeyAction {
  UP,
  DOWN,
}

// action codes
const KEY_UP: usize = 0x0101;
const KEY_DOWN: usize = 0x0100;

impl From<usize> for KeyAction {
  fn from(code: usize) -> Self {
    match code {
      KEY_UP => KeyAction::UP,
      KEY_DOWN => KeyAction::DOWN,
      _ => KeyAction::DOWN,
    }
  }
}

impl std::fmt::Debug for KeyAction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      KeyAction::UP => write!(f, "UP"),
      KeyAction::DOWN => write!(f, "DOWN"),
    }
  }
}

// Key
#[derive(Clone, PartialEq)]
pub struct Key {
  pub code: i32,
  pub name: String,
}

impl Default for Key {
  fn default() -> Self {
    Key {
      code: 0,
      name: "".to_string(),
    }
  }
}

// key codes
pub const KEY_SPACE: i32 = VirtualKey::Space.0;
pub const KEY_WINDOWS: i32 = VirtualKey::LeftWindows.0;
pub const KEY_CONTROL: i32 = VirtualKey::LeftControl.0;
pub const KEY_ALT: i32 = VirtualKey::LeftMenu.0;
pub const KEY_SHIFT: i32 = VirtualKey::LeftShift.0;
pub const MODIFIER_KEYS: [i32; 4] = [KEY_CONTROL, KEY_ALT, KEY_SHIFT, KEY_WINDOWS];

impl From<i32> for Key {
  fn from(code: i32) -> Self {
    let name = match code {
      KEY_SPACE => "SPACE".to_string(),
      KEY_WINDOWS => "WIN".to_string(),
      KEY_CONTROL => "CTRL".to_string(),
      KEY_ALT => "ALT".to_string(),
      KEY_SHIFT => "SHIFT".to_string(),
      _ => char::from(code as u8).to_string(),
    };

    Key {
      code,
      name,
    }
  }
}

impl std::fmt::Debug for Key {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

// Key press
#[derive(Clone, PartialEq)]
pub struct KeyPress {
  pub action: KeyAction,
  pub key: Key,
}

impl KeyPress {
  pub fn new(action: KeyAction, key: Key) -> Self {
    KeyPress {
      action,
      key,
    }
  }
}

impl std::fmt::Debug for KeyPress {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {:?}", self.key.name, self.action)
  }
}
