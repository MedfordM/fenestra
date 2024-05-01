use std::cell::RefCell;
use std::sync::Arc;

use crate::data::key::{Key, Keybind};
use crate::data::monitor::Monitor;

pub mod init;
pub static mut HANDLE: isize = 0;
pub static mut MONITORS: Vec<Arc<RefCell<Monitor>>> = Vec::new();
pub static mut HOOKS: Vec<(String, isize)> = Vec::new();
pub static mut KEYBINDS: Vec<Keybind> = Vec::new();
