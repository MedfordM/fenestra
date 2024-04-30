use std::sync::{Mutex, RwLock};

use lazy_static::lazy_static;
use windows::Win32::Foundation::HWND;

use crate::data::key::{Key, Keybind};
use crate::data::monitor::Monitor;

pub mod init;
lazy_static! {
    pub static ref HANDLE: HWND = init::window();
    pub static ref HOOKS: Vec<(String, isize)> = init::hooks();
    pub static ref KEYBINDS: Vec<Keybind> = init::keybinds();
    pub static ref PRESSED_KEYS: Mutex<Vec<Key>> = Mutex::new(Vec::new());
    pub static ref MONITORS: RwLock<Vec<Monitor>> = RwLock::new(Vec::new());
}
