use std::process::exit;

use windows::core::Error;
use windows::Win32::Foundation::{GetLastError, WIN32_ERROR};
use windows::Win32::UI::WindowsAndMessaging::{HHOOK, UnhookWindowsHookEx};

pub fn handle_result<T>(result: Result<T, Error>) -> T {
    if result.is_err() {
        let error: WIN32_ERROR = unsafe { GetLastError() };
        println!(
            "Encountered an error executing an external DLL function: {:?}",
            error
        );
        exit(error.0 as i32);
    }
    return result.unwrap().into();
}

pub fn unset_hook(hook: &HHOOK) {
    let result = unsafe { UnhookWindowsHookEx(hook.to_owned()) };
    if result.is_err() {
        println!("Failed to unset hooks");
    }
}
