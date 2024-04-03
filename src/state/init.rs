use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::hooks;
use crate::util;

const APP_NAME: &str = "WindowManager\0";

pub fn application() -> HWND {
    let app_instance: HMODULE = util::get_handle();
    util::register_class(app_instance, APP_NAME);

    return util::create_window(
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
}

pub fn state() -> Vec<HHOOK> {
    let hooks: Vec<HHOOK> = hooks::set_hooks();
    return hooks;
}
