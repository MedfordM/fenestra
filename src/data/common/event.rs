use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::WM_APP;

pub struct Event {
    pub message: u32,
    pub wparam: WPARAM,
    pub lparam: LPARAM,
}

pub const KEY_EVENT: u32 = WM_APP + 2;
pub const WINDOW_EVENT: u32 = WM_APP + 3;
pub const MINIMIZE: usize = 0;
pub const RESTORE: usize = 1;
pub const MOVE_SIZE: usize = 2;
pub const FOCUS: usize = 3;
pub const CREATE: usize = 4;
pub const DESTROY: usize = 5;
impl Event {
    pub fn key_event(key_code: isize, wparam: WPARAM) -> Event {
        Event {
            message: KEY_EVENT,
            wparam,
            lparam: LPARAM(key_code),
        }
    }

    pub fn minimize(hwnd: HWND) -> Event {
        Event {
            message: WINDOW_EVENT,
            wparam: WPARAM(MINIMIZE),
            lparam: LPARAM(hwnd.0),
        }
    }

    pub fn restore(hwnd: HWND) -> Event {
        Event {
            message: WINDOW_EVENT,
            wparam: WPARAM(RESTORE),
            lparam: LPARAM(hwnd.0),
        }
    }

    pub fn move_size(hwnd: HWND) -> Event {
        Event {
            message: WINDOW_EVENT,
            wparam: WPARAM(MOVE_SIZE),
            lparam: LPARAM(hwnd.0),
        }
    }

    pub fn focus(hwnd: HWND) -> Event {
        Event {
            message: WINDOW_EVENT,
            wparam: WPARAM(FOCUS),
            lparam: LPARAM(hwnd.0),
        }
    }

    pub fn destroy(hwnd: HWND) -> Event {
        Event {
            message: WINDOW_EVENT,
            wparam: WPARAM(DESTROY),
            lparam: LPARAM(hwnd.0),
        }
    }
}
