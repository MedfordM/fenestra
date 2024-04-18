use std::sync::Mutex;

use lazy_static::lazy_static;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::HHOOK;

use crate::data::key::{Key, Keybind};
use crate::data::monitor::Monitor;
use crate::data::workspace::Workspace;

mod init;
lazy_static! {
    pub static ref HANDLE: HWND = init::window();
    pub static ref HOOKS: Vec<HHOOK> = init::hooks();
    pub static ref KEYBINDS: Vec<Keybind> = init::keybinds();
    pub static ref PRESSED_KEYS: Mutex<Vec<Key>> = Mutex::new(Vec::new());
    pub static ref MONITORS: Mutex<Vec<Monitor>> = Mutex::new(init::monitors());
    pub static ref WORKSPACES: Mutex<Vec<Box<Workspace>>> = Mutex::new(init::workspaces());
}
