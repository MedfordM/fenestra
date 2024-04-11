use std::collections::HashSet;
use std::sync::Mutex;

use lazy_static::lazy_static;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::HHOOK;

use crate::data::key::{Key, Keybind};

mod init;
lazy_static! {
    pub static ref HANDLE: HWND = init::window();
    pub static ref HOOKS: Vec<HHOOK> = init::hooks();
    pub static ref KEYBINDS: Vec<Keybind> = init::keybinds();
    pub static ref PRESSED_KEYS: Mutex<HashSet<Key>> = Mutex::new(HashSet::new());
}
