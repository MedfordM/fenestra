use std::ffi::CString;
use std::process::exit;

use log::error;
use windows::core::PCSTR;
use windows::Win32::Foundation::{
    BOOL, GetLastError, HINSTANCE, HMODULE, HWND, LPARAM, LRESULT, WIN32_ERROR, WPARAM,
};
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::System::StationsAndDesktops::EnumDesktopWindows;
use windows::Win32::UI::WindowsAndMessaging::{BringWindowToTop, CreateWindowExA, CS_HREDRAW, CS_OWNDC, CS_VREDRAW, DefWindowProcA, DispatchMessageA, GetForegroundWindow, GetMessageA, GetWindowInfo, GetWindowLongA, GetWindowTextA, GetWindowThreadProcessId, GWL_STYLE, HCURSOR, HHOOK, IDC_ARROW, LoadCursorW, MSG, PostQuitMessage, RegisterClassA, SetWindowPlacement, SetWindowPos, ShowWindow, SW_SHOW, SWP_FRAMECHANGED, TranslateMessage, WINDOW_EX_STYLE, WINDOW_LONG_PTR_INDEX, WINDOW_STYLE, WINDOWINFO, WINDOWPLACEMENT, WM_DESTROY, WM_NULL, WM_PAINT, WNDCLASSA, WS_CAPTION, WS_MAXIMIZEBOX, WS_VISIBLE};


use crate::data::window::Window;
use crate::hooks;
use crate::state::MONITORS;
use crate::win_api::misc::{attach_thread, detach_thread, handle_result};
use crate::win_api::monitor::get_monitor_from_window;

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
        error!("Error registering window class: {:?}", error);
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
            break;
        }
    }
}

pub fn get_foreground_window() -> Window {
    unsafe {
        let result = GetForegroundWindow();
        if result == HWND::default() {
            let error: WIN32_ERROR = GetLastError();
            error!("Error getting foreground window: {:?}", error);
        }
        return Window::from(result);
    }
}

pub fn set_foreground_window(app: &Window) {
    unsafe {
        let current_window = GetWindowThreadProcessId(GetForegroundWindow(), None);
        attach_thread(current_window);
        BringWindowToTop(app.hwnd).unwrap();
        ShowWindow(app.hwnd, SW_SHOW);
        detach_thread(current_window);
    }
}

pub fn get_style(handle: HWND) -> i32 {
    return get_window_info(handle, GWL_STYLE);
}

static mut WINDOWS: Vec<Window> = Vec::new();
pub fn get_all() -> Vec<Window> {
    unsafe {
        WINDOWS.clear();
    }
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
        let application: Window = Window::from(hwnd);
        if application.title.len() == 0 {
            return BOOL::from(true);
        }
        unsafe {
            WINDOWS.push(application);
        };
        return BOOL::from(true);
    }
    handle_result(unsafe {
        EnumDesktopWindows(None, Some(enum_windows_callback), LPARAM::default())
    });
    unsafe {
        return WINDOWS.clone();
    }
}

pub fn get_window(hwnd: HWND) -> Window {
    let title: String = get_window_title(hwnd);
    let window_info: WINDOWINFO = get_window_coords(hwnd);
    let monitor_result = MONITORS
        .lock()
        .unwrap()
        .iter()
        .find(|monitor| monitor.hmonitor == get_monitor_from_window(hwnd))
        .map(|monitor| monitor.clone());
    if monitor_result.is_none() {
        error!(
            "Unable to find monitor for window {}, {:?}",
            &title, &window_info.rcWindow
        );
        exit(100);
    }
    let monitor = monitor_result.unwrap().clone();
    let (thread_id, process_id) = get_window_thread_id(hwnd);
    Window {
        title,
        hwnd,
        thread_id,
        process_id,
        info: window_info,
        monitor,
    }
}

pub fn set_window_pos(window: &Window) {
    handle_result(unsafe { SetWindowPos(
        window.hwnd,
        None,
        window.info.rcWindow.left - window.info.cxWindowBorders as i32,
        window.info.rcWindow.top - window.info.cyWindowBorders as i32,
        window.info.rcWindow.right,
        window.info.rcWindow.bottom,
        SWP_FRAMECHANGED
    )});
}

fn get_window_coords(hwnd: HWND) -> WINDOWINFO {
    let mut window_info: WINDOWINFO = WINDOWINFO::default();
    handle_result(unsafe { GetWindowInfo(hwnd, &mut window_info) });
    return window_info;
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

fn get_window_thread_id(handle: HWND) -> (u32, u32) {
    let mut process_id = 0;
    let result = unsafe { GetWindowThreadProcessId(handle, Some(&mut process_id)) };
    if result == 0 {
        error!("Unable to get window pid");
    }
    return (result, process_id);
}

fn load_cursor() -> HCURSOR {
    return handle_result(unsafe { LoadCursorW(None, IDC_ARROW) });
}

fn get_message(message: *mut MSG, window_handle: &HWND) -> BOOL {
    return unsafe { GetMessageA(message, window_handle.to_owned(), 0, 0) };
}

fn get_window_info(handle: HWND, offset: WINDOW_LONG_PTR_INDEX) -> i32 {
    let info = unsafe { GetWindowLongA(handle, offset) };
    return info;
}
