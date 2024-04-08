use crate::data::state::ApplicationState;

mod hooks;
mod data;
mod windows;
mod util;
mod state;
mod config;

fn main() {
    static APP_STATE: ApplicationState = APP_STATE;
    util::handle_events(&APP_STATE.handle, &APP_STATE.hooks);
}