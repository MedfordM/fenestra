use crate::state::{HANDLE, HOOKS};

mod config;
mod data;
mod hooks;
mod state;
mod win_api;
mod actions;

fn main() {
    win_api::window::handle_window_events(&HANDLE, &HOOKS);
}
