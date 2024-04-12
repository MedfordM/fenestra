use crate::state::{HANDLE, HOOKS};

mod actions;
mod config;
mod data;
mod hooks;
mod state;
mod win_api;

fn main() {
    env_logger::init();
    win_api::window::handle_window_events(&HANDLE, &HOOKS);
}
