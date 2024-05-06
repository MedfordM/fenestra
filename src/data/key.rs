use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use windows::System::VirtualKey;

use crate::data::action::WindowManagerAction;
use crate::win_api::keyboard::{get_key_code, get_key_name};

pub enum KeyEventType {
    PRESS,
    RELEASE,
}

impl Debug for KeyEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                KeyEventType::PRESS => "pressed",
                KeyEventType::RELEASE => "released",
            }
        )
    }
}

// action codes
const KEY_UP: usize = 0x0101;
const KEY_DOWN: usize = 0x0100;

impl From<usize> for KeyEventType {
    fn from(code: usize) -> Self {
        match code {
            KEY_UP => KeyEventType::RELEASE,
            KEY_DOWN => KeyEventType::PRESS,
            _ => KeyEventType::RELEASE,
        }
    }
}

pub struct Key {
    pub code: i32,
    pub name: String,
}

impl Debug for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Eq for Key {}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.code.partial_cmp(&other.code)
    }
}

impl Ord for Key {
    fn cmp(&self, other: &Self) -> Ordering {
        self.code.cmp(&other.code)
    }
}

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

pub struct KeyEvent {
    pub event: KeyEventType,
    pub key: Key,
}

impl Debug for KeyEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeyEvent {{ '{:?}', '{:?}' }}", &self.key, &self.event)
    }
}

impl KeyEvent {
    pub fn new(event: KeyEventType, key: Key) -> Self {
        KeyEvent { event, key }
    }
}

pub struct Keybind {
    pub keys: Vec<Key>,
    pub action: WindowManagerAction,
}

impl Keybind {
    pub fn new(keys: Vec<Key>, action: WindowManagerAction) -> Self {
        Keybind { keys, action }
    }
}
