use windows::Win32::{Foundation::{GetLastError, HWND, WIN32_ERROR}, UI::WindowsAndMessaging::GetForegroundWindow};

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

pub fn set_foreground_window() {
  
}