use std::path::Path;

use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::config;
use crate::data::key::Keybind;
use crate::data::monitor::Monitor;
use crate::hooks;
use crate::win_api;

const APP_NAME: &str = "WindowManager\0";

pub fn window() -> HWND {
    let app_instance: HMODULE = win_api::misc::get_main_module();
    win_api::window::register_class(app_instance, APP_NAME);

    return win_api::window::create_window(
        WINDOW_EX_STYLE::default(),
        APP_NAME,
        APP_NAME,
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        400,
        680,
        app_instance,
    );
}

pub fn hooks() -> Vec<HHOOK> {
    let hooks: Vec<HHOOK> = hooks::set_hooks();
    return hooks;
}

pub fn keybinds() -> Vec<Keybind> {
    let config_path: &Path = Path::new("./fenestra.conf");
    config::load::ensure_exists(config_path);
    let configured_key_binds: Vec<Keybind> = config::parse::parse_content(config_path);
    return configured_key_binds;
}

pub fn monitors() -> Vec<Monitor> {
    let monitors: Vec<Monitor> = Monitor::get_all_monitors();
    return monitors;
}
