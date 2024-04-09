use crate::data::key::Keybind;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::HHOOK;

pub struct ApplicationState {
    pub handle: HWND,
    pub hooks: Vec<HHOOK>,
    pub keybinds: Vec<Keybind>,
}

impl ApplicationState {
    pub const fn new(handle: HWND, hooks: Vec<HHOOK>, keybinds: Vec<Keybind>) -> ApplicationState {
        ApplicationState {
            handle,
            hooks,
            keybinds,
        }
    }
}
