use windows::Win32::{Foundation::{GetLastError, HWND, WIN32_ERROR}, UI::WindowsAndMessaging::GetForegroundWindow};
use windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow;

pub fn get_foreground_window() -> HWND {
    unsafe {
        let window_handle = GetForegroundWindow();
        if window_handle == HWND::default() {
            println!("Error getting foreground window");
            let error: WIN32_ERROR = GetLastError();
            println!("Error code: {:?}", error);
        }
        return window_handle;
    }
}

pub fn set_foreground_window(window_handle: HWND) {
    let result = unsafe { SetForegroundWindow(window_handle) };
    if result.as_bool() {
        println!("Set foreground window");
    } else {
        println!("Error setting foreground window");
        let error: WIN32_ERROR = unsafe { GetLastError() };
        println!("Error code: {:?}", error);
    }
}