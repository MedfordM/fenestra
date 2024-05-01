use crate::win_api::window::set_dpi_awareness;

mod actions;
mod config;
mod data;
mod hooks;
mod state;
mod win_api;

fn main() {
    env_logger::init();
    set_dpi_awareness();
    state::init::application();
    win_api::window::handle_window_events();
}
