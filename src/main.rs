use crate::state::{HANDLE, HOOKS};

mod hooks;
mod data;
mod windows;
mod util;
mod state;
mod config;

fn main() {
    util::handle_events(&HANDLE, &HOOKS);
}