 use windows::{core::PCSTR, Win32::{Foundation::{GetLastError, HWND, WIN32_ERROR}, UI::WindowsAndMessaging::{CreateWindowExA, GetMessageA, MSG, WINDOW_EX_STYLE, WM_CLOSE, WM_QUIT, WS_TILED}}};

mod hook_control;
mod data;
pub mod window_control;

fn main() {
    let hook_id = hook_control::set_hooks();
    let window_class = PCSTR(b"Button\0".as_ptr());
    let window_name: PCSTR = PCSTR("Rust window\0".as_ptr());

    unsafe {
        let window_handle = CreateWindowExA(WINDOW_EX_STYLE::default(),  window_class, window_name, WS_TILED, 0, 0, 0, 0, None, None, None, None);
        if window_handle == HWND::default() {
            println!("Error creating window");
            let error: WIN32_ERROR = GetLastError();
            println!("Error code: {:?}", error);
        }

        let mut message: MSG = MSG::default();
        while GetMessageA(&mut message, window_handle,  0, 0).as_bool() {
            if message.message == WM_QUIT  || message.message == WM_CLOSE {
                print!("Got quit message");
                hook_control::unset_hooks(hook_id);
                break;
            }
        }
    }
}