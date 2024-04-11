use std::process::exit;

use windows::core::Error;
use windows::Win32::Foundation::{GetLastError, HMODULE, LPARAM, LRESULT, WIN32_ERROR, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
use windows::Win32::UI::WindowsAndMessaging::{CallNextHookEx, UnhookWindowsHookEx, HHOOK};

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

pub fn call_next_hook(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    return unsafe { CallNextHookEx(None, code, w_param, l_param) };
}

fn get_current_thread_id() -> u32 {
    let result = unsafe { GetCurrentThreadId() };
    if result == 0 {
        println!("Unable to get current thread id");
    }
    return result;
}

pub fn attach_thread(target_thread: u32) {
    let fenestra_thread = get_current_thread_id();
    unsafe { AttachThreadInput(target_thread, fenestra_thread, true) };
}

pub fn detach_thread(target_thread: u32) {
    let fenestra_thread = get_current_thread_id();
    unsafe { AttachThreadInput(target_thread, fenestra_thread, false) };
}

pub fn get_main_module() -> HMODULE {
    return handle_result(unsafe { GetModuleHandleA(None) });
}
