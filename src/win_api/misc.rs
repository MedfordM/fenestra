use std::process::exit;

use log::error;
use windows::core::Error;
use windows::Win32::Foundation::{GetLastError, HMODULE, WIN32_ERROR};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};

pub fn handle_result<T>(result: Result<T, Error>) -> T {
    if result.is_err() {
        let error: WIN32_ERROR = unsafe { GetLastError() };
        error!(
            "Encountered an error executing an external DLL function: {:?}",
            error
        );
        exit(error.0 as i32);
    }
    return result.unwrap().into();
}

fn get_current_thread_id() -> u32 {
    let result = unsafe { GetCurrentThreadId() };
    if result == 0 {
        error!("Unable to get current thread id");
    }
    return result;
}

pub fn attach_thread(target_thread: u32) {
    let fenestra_thread = get_current_thread_id();
    let _ = unsafe { AttachThreadInput(target_thread, fenestra_thread, true) };
}

pub fn detach_thread(target_thread: u32) {
    let fenestra_thread = get_current_thread_id();
    let _ = unsafe { AttachThreadInput(target_thread, fenestra_thread, false) };
}

pub fn get_main_module() -> HMODULE {
    return handle_result(unsafe { GetModuleHandleA(None) });
}
