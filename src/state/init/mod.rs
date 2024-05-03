mod monitors;
mod workspace;

use crate::data::common::state::AppState;
use crate::data::key::Keybind;
use crate::data::monitor::Monitor;
use crate::state::init::monitors::init_workspaces;
use crate::win_api::window::set_dpi_awareness;
use crate::{config, hooks, win_api};
use std::cell::RefCell;
use std::path::Path;
use std::sync::Arc;
use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::UI::WindowsAndMessaging::{
    CW_USEDEFAULT, WINDOW_EX_STYLE, WS_OVERLAPPEDWINDOW,
};

pub fn application() -> AppState {
    AppState::new(window(), hooks(), keybinds(), monitors())
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

fn monitors() -> Vec<Arc<RefCell<Monitor>>> {
    let monitors = win_api::monitor::get_all();
    let arc_monitors = monitors::init_neighbors(monitors);
    init_workspaces(&arc_monitors);
    return arc_monitors;
}
