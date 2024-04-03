use ::windows::Win32::Foundation::HWND;
use ::windows::Win32::UI::WindowsAndMessaging::HHOOK;

mod hooks;
mod data;
mod windows;
mod util;
mod state;
mod config;

fn main() {
    let app_handle: HWND = state::init::application();
    let app_hooks: Vec<HHOOK> = state::init::state();
    util::handle_events(app_handle, app_hooks);
}