use std::path::Path;
use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::hooks;
use crate::util;
use crate::config;

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
        400,
        680,
        app_instance
    );
}

pub fn state() -> Vec<HHOOK> {
    let config_path: &Path = Path::new("./fenestra.conf");
    config::load::ensure_exists(config_path);
    config::parse::parse_content(config_path);
    let hooks: Vec<HHOOK> = hooks::set_hooks();
    return hooks;
}
