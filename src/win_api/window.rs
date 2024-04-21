use std::ffi::CString;
use std::process::exit;

use log::error;
use windows::core::PCSTR;
use windows::Win32::Foundation::{
    GetLastError, BOOL, HINSTANCE, HMODULE, HWND, LPARAM, LRESULT, POINT, RECT, WIN32_ERROR, WPARAM
};
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::System::StationsAndDesktops::EnumDesktopWindows;
use windows::Win32::UI::Shell::{Shell_NotifyIconA, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NOTIFYICONDATAA};
use windows::Win32::UI::WindowsAndMessaging::{BringWindowToTop, CreatePopupMenu, CreateWindowExA, DefWindowProcA, DestroyMenu, DispatchMessageA, GetCursorPos, GetForegroundWindow, GetMessageA, GetWindowInfo, GetWindowLongA, GetWindowPlacement, GetWindowRect, GetWindowTextA, GetWindowThreadProcessId, InsertMenuA, LoadCursorW, LoadIconW, PostMessageA, PostQuitMessage, RegisterClassA, SetForegroundWindow, SetWindowPos, ShowWindow, TrackPopupMenu, TranslateMessage, CS_HREDRAW, CS_OWNDC, CS_VREDRAW, GWL_STYLE, HCURSOR, HHOOK, IDC_ARROW, IDI_APPLICATION, MF_STRING, MSG, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOCOPYBITS, SWP_NOSENDCHANGING, SW_FORCEMINIMIZE, SW_SHOW, TPM_BOTTOMALIGN, TPM_RIGHTALIGN, TPM_RIGHTBUTTON, WINDOWINFO, WINDOWPLACEMENT, WINDOW_EX_STYLE, WINDOW_LONG_PTR_INDEX, WINDOW_STYLE, WM_APP, WM_COMMAND, WM_DESTROY, WM_NULL, WM_PAINT, WM_RBUTTONUP, WM_USER, WNDCLASSA, WS_CAPTION, WS_MAXIMIZEBOX, WS_VISIBLE};

use crate::data::window::Window;
use crate::hooks;
use crate::state::MONITORS;
use crate::win_api::misc::{attach_thread, detach_thread, handle_result};
use crate::win_api::monitor::get_monitor_from_window;

pub fn system_tray(hwnd: &HWND) {
    // let tooltip_text = "WindowManager".as_bytes();
    // let mut tooltip: [i8;128] = [0;128];
    // for (&x, p) in tooltip_text.iter().zip(tooltip.iter_mut()) {
    //     *p = x as i8;
    // }

    let data = NOTIFYICONDATAA {
        cbSize: Default::default(),
        hWnd: *hwnd,
        uID: 0,
        uFlags:  NIF_TIP | NIF_ICON | NIF_MESSAGE,
        uCallbackMessage: WM_APP + 1,
        hIcon: unsafe { LoadIconW(HMODULE::default(), IDI_APPLICATION).unwrap() },
        szTip: [0;128],
        dwState: Default::default(),
        dwStateMask: Default::default(),
        szInfo: [0;256],
        Anonymous: Default::default(),
        szInfoTitle: [0;64],
        dwInfoFlags: Default::default(),
        guidItem: Default::default(),
        hBalloonIcon: Default::default(),
    };
    let result = unsafe { Shell_NotifyIconA(NIM_ADD, &data) };
    if !result.as_bool() {
        error!("Unable to create system tray icon");
    }
}

pub fn display_tray_menu(hwnd: HWND) {
    let mut item_text: String = String::from("Quit\0");
    unsafe {
		let context_menu = handle_result(CreatePopupMenu());
		handle_result(InsertMenuA(
			context_menu,
			0,
			MF_STRING,
            (WM_USER + 1) as usize,
			PCSTR::from_raw(item_text.as_mut_ptr()),
		));

		let mut cursor_position = POINT { x: 0, y: 0 };
		handle_result(GetCursorPos(&mut cursor_position));

        let _ = SetForegroundWindow(hwnd);
		let flags = TPM_RIGHTALIGN | TPM_BOTTOMALIGN | TPM_RIGHTBUTTON;
		let _ = TrackPopupMenu(
			context_menu,
			flags,
			cursor_position.x,
			cursor_position.y,
			0,
			hwnd.clone(),
			None,
		);
		handle_result(PostMessageA(hwnd, 0, WPARAM(0), LPARAM(0)));
        handle_result(DestroyMenu(context_menu));
	}
}

pub fn register_class(instance: HMODULE, class_name: &str) {
    extern "system" fn window_callback(
        window: HWND,
        message: u32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        const NOTIFICATION: u32 = (WM_APP + 1) as u32;
        match message {
            WM_PAINT => unsafe {
                _ = ValidateRect(window, None);
                LRESULT(0)
            },
            WM_DESTROY => unsafe {
                PostQuitMessage(0);
                LRESULT(0)
            },
            NOTIFICATION => {
                if l_param.0 as u32 == WM_RBUTTONUP {
                    display_tray_menu(window);
                }
                LRESULT(0)
            },
            WM_COMMAND => {
                if w_param.0 == (WM_USER + 1) as usize {
                    unsafe { PostQuitMessage(0) };
                }
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

pub fn handle_window_events(window_handle: &HWND, hook_ids: &Vec<HHOOK>) {
    let mut message: MSG = MSG::default();
    while get_message(&mut message, window_handle).into() {
        unsafe {
            let _ = TranslateMessage(&message);
        }
        unsafe {
            DispatchMessageA(&message);
        }
        if message.message == WM_NULL {
            hooks::unset_hooks(hook_ids);
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
        restore_window(app);
        BringWindowToTop(app.hwnd).unwrap();
        //let _ = ShowWindow(app.hwnd, SW_SHOW);
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
    let window_rect: RECT = get_rect(hwnd);
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
    let window_placement: WINDOWPLACEMENT = get_window_placement(hwnd);
    Window {
        title,
        hwnd,
        thread_id,
        process_id,
        rect: window_rect,
        info: window_info,
        placement: window_placement,
        monitor,
    }
}

pub fn set_window_pos(window: &Window) {
    // debug!("Setting position for '{}' from {:?} to {:?}", &window.title, &window.info.rcWindow, window.);
    handle_result(unsafe {
        SetWindowPos(
            window.hwnd,
            None,
            window.rect.left - window.info.cxWindowBorders as i32,
            window.rect.top - window.info.cyWindowBorders as i32,
            window.rect.right + window.info.cxWindowBorders as i32,
            window.rect.bottom + window.info.cyWindowBorders as i32,
            SWP_NOACTIVATE | SWP_NOSENDCHANGING | SWP_NOCOPYBITS | SWP_FRAMECHANGED,
        )
    });
}

pub fn minimize_window(window: &Window) {
    let result = unsafe { ShowWindow(window.hwnd, SW_FORCEMINIMIZE) };
    if !result.as_bool() {
        error!("Unable to minimize window {}", window.title);
    }
}

pub fn restore_window(window: &Window) {
    let result = unsafe { ShowWindow(window.hwnd, SW_SHOW) };
    if !result.as_bool() {
        error!("Unable to restore window {}", window.title);
    }
}

// pub fn set_window_placement(window: &Window, new_placement: &WINDOWPLACEMENT) {
//     let placement: WINDOWPLACEMENT = WINDOWPLACEMENT {
//         length: new_placement.length,
//         flags: WPF_ASYNCWINDOWPLACEMENT,
//         showCmd: SW_SHOW.0 as u32,
//         ptMinPosition: new_placement.ptMinPosition,
//         ptMaxPosition: new_placement.ptMaxPosition,
//         rcNormalPosition: new_placement.rcNormalPosition,
//     };
//     debug!("Setting '{}' position to {:?}", &window.title, &placement);
//     handle_result(unsafe { SetWindowPlacement(window.hwnd, &placement as *const WINDOWPLACEMENT) });
// }

pub fn get_window_placement(hwnd: HWND) -> WINDOWPLACEMENT {
    let mut window_placement: WINDOWPLACEMENT = WINDOWPLACEMENT::default();
    let _ = handle_result(unsafe { GetWindowPlacement(hwnd, &mut window_placement) });
    return window_placement;
}

fn get_window_coords(hwnd: HWND) -> WINDOWINFO {
    let mut window_info: WINDOWINFO = WINDOWINFO::default();
    handle_result(unsafe { GetWindowInfo(hwnd, &mut window_info) });
    return window_info;
}

fn get_rect(hwnd: HWND) -> RECT {
    let mut rect = RECT::default();
    handle_result(unsafe { GetWindowRect(hwnd, &mut rect) });
    return rect;
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
