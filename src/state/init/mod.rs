mod monitors;

use crate::data::common::state::AppState;
use crate::data::hook::Hook;
use crate::data::key::Keybind;
use crate::data::monitor::Monitor;
use crate::win_api::window::set_dpi_awareness;
use crate::{config, hooks, win_api};
use std::path::Path;
use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::UI::WindowsAndMessaging::{
    CW_USEDEFAULT, WINDOW_EX_STYLE, WS_OVERLAPPEDWINDOW,
};

pub fn application() -> AppState {
    AppState::new(window(), hooks())
}

fn window() -> HWND {
    set_dpi_awareness();
    let app_instance: HMODULE = win_api::misc::get_main_module();
    const APP_NAME: &str = "WindowManager\0";
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
fn hooks() -> Vec<Box<dyn Hook>> {
    hooks::set_hooks()
}

pub fn keybinds() -> Vec<Keybind> {
    let config_path: &Path = Path::new("./fenestra.conf");
    config::load::ensure_exists(config_path);
    let configured_key_binds: Vec<Keybind> = config::parse::parse_content(config_path);
    return configured_key_binds;
}

pub fn monitors() -> Vec<Monitor> {
    let monitors = win_api::monitor::get_all();
    return monitors::init_neighbors(monitors);
}
