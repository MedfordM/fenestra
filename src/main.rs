use crate::win_api::window::set_dpi_awareness;
use state::{HANDLE, HOOKS};

mod actions;
mod config;
mod data;
mod hooks;
mod state;
mod win_api;

fn main() {
    env_logger::init();
    set_dpi_awareness();
    state::init::monitors();
    win_api::window::handle_window_events(&HANDLE, &HOOKS);
}
