use std::path::Path;

use windows::Win32::Foundation::{HMODULE, HWND};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::config;
use crate::data::key::Keybind;
use crate::data::monitor::Monitor;
use crate::data::window::Window;
use crate::data::workspace::Workspace;
use crate::hooks;
use crate::win_api;

const APP_NAME: &str = "WindowManager\0";

pub fn window() -> HWND {
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

pub fn workspaces() -> Vec<Box<Workspace>> {
    let mut workspaces: Vec<Box<Workspace>> = vec![];
    let default_workspace: Workspace = Workspace {
        id: 1,
        focused: true,
        windows: Window::get_all_windows()
    };
    workspaces.push(Box::new(default_workspace));
    for i in 2..10 {
        let workspace: Workspace = Workspace {
            id: i,
            focused: false,
            windows: vec![],
        };
        workspaces.push(Box::new(workspace));
    }
    return workspaces;
}
