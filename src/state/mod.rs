use std::cell::RefCell;
use std::sync::Arc;

use crate::data::key::{Key, Keybind};
use crate::data::monitor::Monitor;

mod init;
mod monitor;
pub mod state_manager;
mod validation;
mod management;
