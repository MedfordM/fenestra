use std::str::FromStr;

use windows::System::VirtualKey;

use crate::data::action::WindowManagerAction;
use crate::win_api::keyboard::{get_key_code, get_key_name};

// Key action
#[derive(Clone, Copy, PartialEq, Eq)]
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
#[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
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
pub const WINDOWS_KEY_CODE: i32 = VirtualKey::LeftWindows.0;
pub const KEY_CONTROL: i32 = VirtualKey::LeftControl.0;
pub const KEY_ALT: i32 = VirtualKey::LeftMenu.0;
pub const KEY_SHIFT: i32 = VirtualKey::LeftShift.0;


impl From<i32> for Key {
    fn from(code: i32) -> Self {
        let name = match code {
            KEY_SPACE => "SPACE".to_string(),
            WINDOWS_KEY_CODE => "WIN".to_string(),
            KEY_CONTROL => "CTRL".to_string(),
            KEY_ALT => "ALT".to_string(),
            KEY_SHIFT => "SHIFT".to_string(),
            _ => String::from(get_key_name(code)),
        };

        Key { code, name }
    }
}

impl FromStr for Key {
    type Err = ();

    fn from_str(key: &str) -> Result<Self, Self::Err> {
        let key_code: i32 = get_key_code(key);
        Ok(Key::from(key_code))
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
        KeyPress { action, key }
    }
}

impl std::fmt::Debug for KeyPress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.key.name, self.action)
    }
}

#[derive(Clone, PartialEq)]
pub struct Keybind {
    pub keys: Vec<Key>,
    pub action: WindowManagerAction,
}

impl Keybind {
    pub fn new(keys: Vec<Key>, action: WindowManagerAction) -> Self {
        Keybind { keys, action }
    }
}

impl std::fmt::Debug for Keybind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {:?}", self.action, self.keys)
    }
}
