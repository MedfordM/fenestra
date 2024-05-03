use crate::state::state_manager::StateManager;

mod actions;
mod config;
mod data;
mod hooks;
mod state;
mod win_api;

fn main() {
    env_logger::init();
    let state_manager = StateManager::new();
    state_manager.handle_window_events();
}
