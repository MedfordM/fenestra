use std::path::Path;

use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::{config, win_api};
use crate::data::key::Keybind;
use crate::hooks;

const APP_NAME: &str = "WindowManager\0";

pub fn application() {
    window();
    hooks();
    keybinds();
    monitors();
}

fn window() -> HWND {
    let app_instance: HMODULE = win_api::misc::get_main_module();
    win_api::window::register_class(app_instance, APP_NAME);

    let hwnd: HWND = win_api::window::create_window(
        WINDOW_EX_STYLE::default(),
        APP_NAME,
        APP_NAME,
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        400,
        680,
        app_instance,
    );

    win_api::window::system_tray(&hwnd);
    return hwnd;
}

fn hooks() -> Vec<(String, isize)> {
    let hooks: Vec<(String, isize)> = hooks::set_hooks();
    return hooks;
}

fn keybinds() -> Vec<Keybind> {
    let config_path: &Path = Path::new("./fenestra.conf");
    config::load::ensure_exists(config_path);
    let configured_key_binds: Vec<Keybind> = config::parse::parse_content(config_path);
    return configured_key_binds;
}

fn monitors() {
    win_api::monitor::get_all();
}
