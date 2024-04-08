use lazy_static::lazy_static;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::HHOOK;
use crate::data::key::Keybind;
use crate::data::state::ApplicationState;

mod init;
lazy_static! {
    pub static ref APP_STATE: ApplicationState = {
        static ref app_handle: HWND = init::window();
        let app_hooks: Vec<HHOOK> = init::hooks();
        let app_keybinds: Vec<Keybind> = init::keybinds();
        let state = ApplicationState::new(app_handle, app_hooks, app_keybinds);
        state
    };
}