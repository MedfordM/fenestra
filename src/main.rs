use crate::state::{HANDLE, HOOKS};

mod config;
mod data;
mod hooks;
mod state;
mod util;
mod win_api;

fn main() {
    win_api::window::handle_window_events(&HANDLE, &HOOKS);
}
