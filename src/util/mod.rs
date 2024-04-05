use std::ffi::CString;
use std::process::exit;

use windows::core::{Error, PCSTR};
use windows::Win32::Foundation::{BOOL, GetLastError, HINSTANCE, HMODULE, HWND, LPARAM, LRESULT, WIN32_ERROR, WPARAM};
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyNameTextA, MapVirtualKeyA, MAPVK_VK_TO_VSC, VkKeyScanA};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExA, CS_HREDRAW, CS_OWNDC, CS_VREDRAW, DefWindowProcA, DispatchMessageA, GetMessageA, HCURSOR, HHOOK, IDC_ARROW, LoadCursorW, MSG, PostQuitMessage, RegisterClassA, SetWindowsHookExA, TranslateMessage, UnhookWindowsHookEx, WH_KEYBOARD_LL, WINDOW_EX_STYLE, WINDOW_STYLE, WM_DESTROY, WM_NULL, WM_PAINT, WNDCLASSA};

use crate::hooks;
use crate::hooks::hook_keyboard::keyboard_hook::callback;

fn handle_result<T>(result: Result<T, Error>) -> T {
    if result.is_err() {
        let error: WIN32_ERROR = unsafe { GetLastError() };
        println!("Encountered an error executing an external DLL function: {:?}", error);
        exit(error.0 as i32);
    }
    return result.unwrap().into();
}

pub fn get_handle() -> HMODULE {
    return handle_result(unsafe { GetModuleHandleA(None) });
}

pub fn load_cursor() -> HCURSOR {
    return handle_result(unsafe { LoadCursorW(None, IDC_ARROW) });
}

pub fn register_class(instance: HMODULE, class_name: &str) {
    let window_class = WNDCLASSA {
        style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(window_callback),
        hInstance: HINSTANCE(instance.0),
        hCursor: load_cursor(),
        lpszClassName: PCSTR(class_name.as_ptr()),
        ..Default::default()
    };
   let result = unsafe { RegisterClassA(&window_class) };
    if result == 0 {
        println!("Error registering window class");
        let error: WIN32_ERROR = unsafe { GetLastError() };
        println!("Error code: {:?}", error);
    }
}

extern "system" fn window_callback(window: HWND, message: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match message {
        WM_PAINT =>  unsafe {
            _ = ValidateRect(window, None);
            LRESULT(0)
        }
        WM_DESTROY => unsafe {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcA(window, message, w_param, l_param) }
    }
}

pub fn create_window(ex_style: WINDOW_EX_STYLE, class_name: &str, window_name: &str, style: WINDOW_STYLE, x: i32, y: i32, width: i32, height: i32, instance: HMODULE) -> HWND {
    return unsafe {
        CreateWindowExA(
            ex_style,
            PCSTR(class_name.as_ptr()),
            PCSTR(window_name.as_ptr()),
            style,
            x,
            y,
            width,
            height,
            None,
            None,
            instance,
            None
        )
    };
}

fn get_message(message: *mut MSG, window_handle: HWND) -> BOOL {
    return unsafe { GetMessageA(message, window_handle, 0, 0) };
}

pub fn handle_events(window_handle: HWND, hooks: Vec<HHOOK>) {
    let mut message: MSG = MSG::default();
    while get_message(&mut message, window_handle).into() {
        unsafe { TranslateMessage(&message); }
        unsafe { DispatchMessageA(&message); }
        if message.message == WM_NULL {
            hooks::unset_hooks(hooks);
            println!("Exiting");
            break;
        }
    };
}

pub fn set_keyboard_hook() -> HHOOK {
    return handle_result(unsafe { SetWindowsHookExA(WH_KEYBOARD_LL, Some(callback), HINSTANCE(0), 0) });
}

pub fn unset_hook(hook: &HHOOK) {
    let result = unsafe { UnhookWindowsHookEx(hook.to_owned()) };
    if result.is_err() {
        println!("Failed to unset hooks");
    }
}

pub fn get_key_name(key_code: i32) -> String {
    let scan_code = unsafe { MapVirtualKeyA(key_code as u32, MAPVK_VK_TO_VSC) };
    let mut buffer = vec![0; 32];
    let result = unsafe { GetKeyNameTextA((scan_code << 16) as i32, &mut buffer) };

    if result == 0 {
        println!("Failed to fetch key name for code {}", key_code);
        return String::new();
    }
    unsafe { buffer.set_len(result as usize) }
    return CString::new(buffer).unwrap().into_string().unwrap();
}

pub fn get_key_code(key: &str) -> i32 {
    let result = unsafe { VkKeyScanA(key.as_bytes()[0] as i8) };
    if result == 0 {
        println!("Failed to fetch key code for character {}", key);
    }
    result as i32
}