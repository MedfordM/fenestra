use windows::Win32::UI::WindowsAndMessaging::*;

use crate::{hooks, state};
use crate::util;

const APP_NAME: &str = "WindowManager\0";

pub fn init() {
    let app_instance = util::get_handle();
    util::register_class(app_instance, APP_NAME);

    let window_handle = util::create_window(
            WINDOW_EX_STYLE::default(),
            APP_NAME,
            APP_NAME,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            app_instance
    );

    hooks::set_hooks(state::HOOKS);
    util::handle_events(window_handle);
}
