use std::ffi::CString;

use windows::core::PCSTR;
use windows::Win32::Foundation::{
    BOOL, GetLastError, HINSTANCE, HMODULE, HWND, LPARAM, LRESULT, WIN32_ERROR, WPARAM,
};
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::StationsAndDesktops::EnumDesktopWindows;
use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
use windows::Win32::UI::WindowsAndMessaging::{
    BringWindowToTop, CreateWindowExA, CS_HREDRAW, CS_OWNDC, CS_VREDRAW,
    DefWindowProcA, DispatchMessageA, GetForegroundWindow, GetMessageA, GetWindowLongA,
    GetWindowPlacement, GetWindowTextA, GetWindowThreadProcessId, GWL_STYLE, HCURSOR, HHOOK,
    IDC_ARROW, LoadCursorW, MSG, PostQuitMessage, RegisterClassA, ShowWindow, SW_SHOW, TranslateMessage, WINDOW_EX_STYLE,
    WINDOW_LONG_PTR_INDEX, WINDOW_STYLE, WINDOWPLACEMENT, WM_DESTROY, WM_NULL, WM_PAINT, WNDCLASSA,
    WS_CAPTION, WS_MAXIMIZEBOX, WS_VISIBLE,
};

use crate::data::window::Window;
use crate::hooks;
use crate::win_api::misc::handle_result;

pub fn get_handle() -> HMODULE {
    return handle_result(unsafe { GetModuleHandleA(None) });
}

pub fn load_cursor() -> HCURSOR {
    return handle_result(unsafe { LoadCursorW(None, IDC_ARROW) });
}

pub fn register_class(instance: HMODULE, class_name: &str) {
    extern "system" fn window_callback(
        window: HWND,
        message: u32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        match message {
            WM_PAINT => unsafe {
                _ = ValidateRect(window, None);
                LRESULT(0)
            },
            WM_DESTROY => unsafe {
                PostQuitMessage(0);
                LRESULT(0)
            },
            _ => unsafe { DefWindowProcA(window, message, w_param, l_param) },
        }
    }

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
        let error: WIN32_ERROR = unsafe { GetLastError() };
        println!("Error registering window class: {:?}", error);
    }
}

pub fn create_window(
    ex_style: WINDOW_EX_STYLE,
    class_name: &str,
    window_name: &str,
    style: WINDOW_STYLE,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    instance: HMODULE,
) -> HWND {
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
            None,
        )
    };
}

pub fn handle_window_events(window_handle: &HWND, hooks: &Vec<HHOOK>) {
    let mut message: MSG = MSG::default();
    while get_message(&mut message, window_handle).into() {
        unsafe {
            TranslateMessage(&message);
        }
        unsafe {
            DispatchMessageA(&message);
        }
        if message.message == WM_NULL {
            hooks::unset_hooks(hooks);
            println!("Exiting");
            break;
        }
    }
}

fn get_message(message: *mut MSG, window_handle: &HWND) -> BOOL {
    return unsafe { GetMessageA(message, window_handle.to_owned(), 0, 0) };
}

pub fn get_foreground_window() -> Window {
    unsafe {
        let result = GetForegroundWindow();
        if result == HWND::default() {
            let error: WIN32_ERROR = GetLastError();
            println!("Error getting foreground window: {:?}", error);
        }
        return get_application(result);
    }
}

pub fn set_foreground_window(app: Window) {
    unsafe {
        let current_window = GetWindowThreadProcessId(GetForegroundWindow(), None);
        attach_input(current_window);
        BringWindowToTop(app.hwnd).unwrap();
        ShowWindow(app.hwnd, SW_SHOW);
        detach_input(current_window);
    }
}

fn attach_input(target_thread: u32) {
    let fenestra_thread = get_current_thread_id();
    unsafe { AttachThreadInput(target_thread, fenestra_thread, true) };
}

fn detach_input(target_thread: u32) {
    let fenestra_thread = get_current_thread_id();
    unsafe { AttachThreadInput(target_thread, fenestra_thread, false) };
}

fn get_window_info(handle: HWND, offset: WINDOW_LONG_PTR_INDEX) -> i32 {
    let info = unsafe { GetWindowLongA(handle, offset) };
    return info;
}

fn get_style(handle: HWND) -> i32 {
    return get_window_info(handle, GWL_STYLE);
}

static mut APPLICATIONS: Vec<Window> = Vec::new();
pub fn get_applications() -> Vec<Window> {
    unsafe {
        APPLICATIONS.clear();
    };
    extern "system" fn enum_windows_callback(hwnd: HWND, _: LPARAM) -> BOOL {
        let window_style: u32 = get_style(hwnd) as u32;
        if window_style & WS_VISIBLE.0 == 0 {
            return BOOL::from(true);
        }
        if window_style & WS_CAPTION.0 == 0 {
            return BOOL::from(true);
        }
        if window_style & WS_MAXIMIZEBOX.0 == 0 {
            return BOOL::from(true);
        }

        let application: Window = get_application(hwnd);
        if application.title.len() == 0 {
            return BOOL::from(true);
        }

        unsafe {
            APPLICATIONS.push(application);
        };
        return BOOL::from(true);
    }

    handle_result(unsafe {
        EnumDesktopWindows(None, Some(enum_windows_callback), LPARAM::default())
    });
    unsafe { return APPLICATIONS.clone() };
}

pub fn get_application(hwnd: HWND) -> Window {
    let title: String = get_window_title(hwnd);
    let placement: WINDOWPLACEMENT = get_window_placement(hwnd);
    let (thread_id, process_id) = get_window_thread_id(hwnd);

    Window {
        title,
        hwnd,
        thread_id,
        process_id,
        placement,
    }
}

fn get_window_title(handle: HWND) -> String {
    let mut buffer = vec![0; 32];
    let result = unsafe { GetWindowTextA(handle, &mut buffer) };

    if result == 0 {
        return String::new();
    }
    unsafe { buffer.set_len(result as usize) }
    return CString::new(buffer).unwrap().to_string_lossy().to_string();
}

fn get_window_placement(handle: HWND) -> WINDOWPLACEMENT {
    let mut placement = WINDOWPLACEMENT::default();
    handle_result(unsafe { GetWindowPlacement(handle, &mut placement) });
    return placement;
}

pub fn find_nearest_window_in_direction(direction: &String) -> Window {
    let active_window: Window = get_foreground_window();
    let mut nearest_window: Option<Window> = None;
    let all_windows: Vec<Window> = get_applications();

    all_windows.iter().for_each(|current_window| {
        if current_window.placement.rcNormalPosition == active_window.placement.rcNormalPosition {
            return;
        }
        let current: i32;
        let active: i32;
        let nearest: i32;
        match direction.to_ascii_uppercase().as_str() {
            "LEFT" => {
                current = current_window.placement.rcNormalPosition.left;
                active = active_window.placement.rcNormalPosition.left;
                if current > active {
                    return;
                }
                if nearest_window.is_none() {
                    nearest_window = Some(current_window.clone());
                    return;
                }
                nearest = nearest_window
                    .clone()
                    .unwrap()
                    .placement
                    .rcNormalPosition
                    .left;
                if current > nearest {
                    nearest_window = Some(current_window.clone());
                }
            }
            "DOWN" => {
                current = current_window.placement.rcNormalPosition.bottom;
                active = active_window.placement.rcNormalPosition.bottom;
                if current > active {
                    return;
                }
                if nearest_window.is_none() {
                    nearest_window = Some(current_window.clone());
                    return;
                }
                nearest = nearest_window
                    .clone()
                    .unwrap()
                    .placement
                    .rcNormalPosition
                    .bottom;
                if current > nearest {
                    nearest_window = Some(current_window.clone());
                }
            }
            "UP" => {
                current = current_window.placement.rcNormalPosition.top;
                active = active_window.placement.rcNormalPosition.top;
                if current < active {
                    return;
                }
                if nearest_window.is_none() {
                    nearest_window = Some(current_window.clone());
                }
                nearest = nearest_window
                    .clone()
                    .unwrap()
                    .placement
                    .rcNormalPosition
                    .top;
                if current < nearest {
                    nearest_window = Some(current_window.clone());
                }
            }
            "RIGHT" => {
                current = current_window.placement.rcNormalPosition.right;
                active = active_window.placement.rcNormalPosition.right;
                if current < active {
                    return;
                }
                if nearest_window.is_none() {
                    nearest_window = Some(current_window.clone());
                }
                nearest = nearest_window
                    .clone()
                    .unwrap()
                    .placement
                    .rcNormalPosition
                    .right;
                if current < nearest {
                    nearest_window = Some(current_window.clone());
                }
            }
            _ => return,
        }
    });
    return nearest_window.unwrap_or_else(|| active_window);
}

fn get_window_thread_id(handle: HWND) -> (u32, u32) {
    let mut process_id = 0;
    let result = unsafe { GetWindowThreadProcessId(handle, Some(&mut process_id)) };
    if result == 0 {
        println!("Unable to get window pid");
    }
    return (result, process_id);
}

fn get_current_thread_id() -> u32 {
    let result = unsafe { GetCurrentThreadId() };
    if result == 0 {
        println!("Unable to get current thread id");
    }
    return result;
}
