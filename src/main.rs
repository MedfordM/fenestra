use crate::state::{HANDLE, HOOKS};

mod config;
mod data;
mod hooks;
mod state;
mod util;
mod windows;

fn main() {
    util::handle_events(&HANDLE, &HOOKS);
}
