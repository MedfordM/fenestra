use std::ffi::CString;

use log::{debug, error};
use windows::core::PCSTR;
use windows::Win32::Foundation::{
    GetLastError, BOOL, HINSTANCE, HMODULE, HWND, LPARAM, LRESULT, POINT, RECT, WIN32_ERROR, WPARAM,
};
use windows::Win32::Graphics::Dwm::{
    DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS, DWMWA_VISIBLE_FRAME_BORDER_THICKNESS,
    DWMWINDOWATTRIBUTE,
};
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::System::StationsAndDesktops::EnumDesktopWindows;
use windows::Win32::UI::HiDpi::{
    AdjustWindowRectExForDpi, GetDpiForWindow, SetProcessDpiAwarenessContext,
    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
};
use windows::Win32::UI::Shell::{
    Shell_NotifyIconA, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NOTIFYICONDATAA,
};
use windows::Win32::UI::WindowsAndMessaging::{
    BringWindowToTop, CreatePopupMenu, CreateWindowExA, DefWindowProcA, DestroyMenu, GetCursorPos,
    GetForegroundWindow, GetMessageA, GetWindowInfo, GetWindowLongA, GetWindowPlacement,
    GetWindowRect, GetWindowTextA, GetWindowThreadProcessId, InsertMenuA, LoadCursorW, LoadIconW,
    PostMessageA, PostQuitMessage, RegisterClassA, SendMessageA, SetForegroundWindow, SetWindowPos,
    ShowWindow, TrackPopupMenu, CS_HREDRAW, CS_OWNDC, CS_VREDRAW, GWL_EXSTYLE, GWL_STYLE, HCURSOR,
    IDC_ARROW, IDI_APPLICATION, MF_STRING, MSG, SWP_DRAWFRAME, SWP_FRAMECHANGED, SWP_NOACTIVATE,
    SWP_NOCOPYBITS, SWP_NOSENDCHANGING, SWP_SHOWWINDOW, SW_MAXIMIZE, SW_RESTORE,
    SW_SHOWMINNOACTIVE, TPM_BOTTOMALIGN, TPM_RIGHTALIGN, TPM_RIGHTBUTTON, WINDOWINFO,
    WINDOWPLACEMENT, WINDOW_EX_STYLE, WINDOW_LONG_PTR_INDEX, WINDOW_STYLE, WM_APP, WM_COMMAND,
    WM_DESTROY, WM_DPICHANGED, WM_PAINT, WM_RBUTTONUP, WM_USER, WNDCLASSA, WS_OVERLAPPEDWINDOW,
    WS_SIZEBOX, WS_VISIBLE,
};

use crate::data::window::Window;
use crate::win_api::misc::{attach_thread, detach_thread, handle_result};

pub fn set_dpi_awareness() {
    let _ = unsafe { SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) };
}

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
        uFlags: NIF_TIP | NIF_ICON | NIF_MESSAGE,
        uCallbackMessage: WM_APP + 1,
        hIcon: unsafe { LoadIconW(HMODULE::default(), IDI_APPLICATION).unwrap() },
        szTip: [0; 128],
        dwState: Default::default(),
        dwStateMask: Default::default(),
        szInfo: [0; 256],
        Anonymous: Default::default(),
        szInfoTitle: [0; 64],
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
        const NOTIFICATION: u32 = WM_APP + 1;
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
            }
            WM_COMMAND => {
                if w_param.0 == (WM_USER + 1) as usize {
                    unsafe { PostQuitMessage(100) };
                }
                LRESULT(0)
            }
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

pub fn foreground_hwnd() -> HWND {
    unsafe {
        let result = GetForegroundWindow();
        if result == HWND::default() {
            let error: WIN32_ERROR = GetLastError();
            error!("Error getting foreground window: {:?}", error);
        }
        return result;
    };
}

pub fn focus(hwnd: &HWND) -> bool {
    unsafe {
        let process_id = GetWindowThreadProcessId(GetForegroundWindow(), None);
        attach_thread(process_id);
        let result = BringWindowToTop(hwnd.clone());
        detach_thread(process_id);
        return result.is_ok();
    }
}

fn get_style(handle: HWND) -> i32 {
    return get_window_info(handle, GWL_STYLE);
}

fn get_extended_style(handle: HWND) -> i32 {
    return get_window_info(handle, GWL_EXSTYLE);
}

static mut WINDOWS: Vec<Window> = Vec::new();
pub fn get_all() -> Vec<Window> {
    unsafe {
        WINDOWS.clear();
    }
    extern "system" fn enum_windows_callback(hwnd: HWND, _: LPARAM) -> BOOL {
        let application = get_window(hwnd);
        if application.is_some() {
            unsafe { WINDOWS.push(application.unwrap()) };
        }
        return BOOL::from(true);
    }
    handle_result(unsafe {
        EnumDesktopWindows(None, Some(enum_windows_callback), LPARAM::default())
    });
    unsafe {
        let mut windows = WINDOWS.clone();
        windows.sort_by(|a, b| a.hwnd.0.partial_cmp(&b.hwnd.0).unwrap());
        // let titles: Vec<String> = windows
        //     .iter()
        //     .map(|window| String::from(&window.title))
        //     .collect();
        // debug!("Deduplicating {:?}", titles);
        Vec::dedup(&mut windows);
        return windows;
    }
}

pub fn get_window(hwnd: HWND) -> Option<Window> {
    let title: String = get_window_title(hwnd);
    if title.is_empty() || title.to_lowercase().contains("settings") {
        return None;
    }
    let style = get_style(hwnd);
    let style_u32 = style as u32;
    let extended_style = get_extended_style(hwnd);
    if style_u32 & WS_VISIBLE.0 == 0 {
        // debug!("Ignoring window '{}' as it is not visible", title);
        return None;
    }

    if style_u32 & WS_OVERLAPPEDWINDOW.0 == 0 {
        // debug!("Ignoring window '{}' as it is not overlapped", title);
        return None;
    }

    if style_u32 & WS_SIZEBOX.0 == 0 {
        // debug!(
        // "Ignoring window '{}' as it does not have a size-box",
        // title
        // );
        return None;
    }

    // if style_u32 & WS_MINIMIZE.0 != 0 {
    //     return None;
    // }

    let window_info: WINDOWINFO = get_window_coords(hwnd);
    let (rect, shadow_rect) = get_rect(hwnd);
    let mut border_thickness = 0;
    get_ext_attr(
        hwnd,
        DWMWA_VISIBLE_FRAME_BORDER_THICKNESS,
        &mut border_thickness,
    );
    let (thread_id, process_id) = get_window_thread_id(hwnd);
    let window_placement: WINDOWPLACEMENT = get_window_placement(hwnd);
    let dpi = get_dpi(hwnd);
    return Some(Window {
        title,
        hwnd,
        thread_id,
        process_id,
        border_thickness,
        rect,
        shadow_rect,
        info: window_info,
        placement: window_placement,
        dpi,
        style,
        extended_style,
    });
}

pub fn set_position(hwnd: &HWND, position: RECT, dpi_change: bool) {
    let width: i32 = position.right - position.left;
    let height: i32 = position.bottom - position.top;
    handle_result(unsafe {
        SetWindowPos(
            hwnd.clone(),
            None,
            position.left,
            position.top,
            width,
            height,
            SWP_NOACTIVATE | SWP_NOSENDCHANGING | SWP_NOCOPYBITS | SWP_FRAMECHANGED,
        )
    });
    if dpi_change {
        debug!("DPI was changed, sizing the window again");
        handle_result(unsafe {
            SetWindowPos(
                hwnd.clone(),
                None,
                position.left,
                position.top,
                width,
                height,
                SWP_NOACTIVATE | SWP_NOSENDCHANGING | SWP_NOCOPYBITS | SWP_FRAMECHANGED,
            )
        });
    }
    // handle_result(unsafe {
    //     SetWindowPos(
    //         hwnd.clone(),
    //         None,
    //         position.left,
    //         // dpi_adjusted_window.rect.top,
    //         position.top,
    //         position.right,
    //         position.bottom,
    //         // SWP_NOACTIVATE | SWP_NOSENDCHANGING | SWP_NOCOPYBITS | SWP_DRAWFRAME,
    //         SWP_NOACTIVATE | SWP_NOCOPYBITS | SWP_SHOWWINDOW | SWP_DRAWFRAME | SWP_FRAMECHANGED,
    //     )
    // });
}

pub fn minimize(hwnd: &HWND) -> bool {
    unsafe { ShowWindow(hwnd.clone(), SW_SHOWMINNOACTIVE) }.as_bool()
}

pub fn maximize(hwnd: &HWND) -> bool {
    unsafe { ShowWindow(hwnd.clone(), SW_MAXIMIZE) }.as_bool()
}

pub fn restore(hwnd: &HWND) -> bool {
    unsafe { ShowWindow(hwnd.clone(), SW_RESTORE) }.as_bool()
}

fn get_window_placement(hwnd: HWND) -> WINDOWPLACEMENT {
    let mut window_placement: WINDOWPLACEMENT = WINDOWPLACEMENT::default();
    let _ = handle_result(unsafe { GetWindowPlacement(hwnd, &mut window_placement) });
    return window_placement;
}

fn get_ext_attr<T>(hwnd: HWND, attr: DWMWINDOWATTRIBUTE, value: &mut T) {
    handle_result(unsafe {
        DwmGetWindowAttribute(
            hwnd,
            attr,
            (value as *mut T).cast(),
            u32::try_from(std::mem::size_of::<T>()).unwrap(),
        )
    });
}

pub fn get_window_coords(hwnd: HWND) -> WINDOWINFO {
    let mut window_info: WINDOWINFO = WINDOWINFO::default();
    handle_result(unsafe { GetWindowInfo(hwnd, &mut window_info) });
    return window_info;
}

fn get_rect(hwnd: HWND) -> (RECT, RECT) {
    let mut rect = RECT::default();
    let mut shadow_rect = RECT::default();
    handle_result(unsafe { GetWindowRect(hwnd, &mut rect) });
    get_ext_attr(hwnd, DWMWA_EXTENDED_FRAME_BOUNDS, &mut shadow_rect);
    return (rect, shadow_rect);
}

pub fn get_window_title(handle: HWND) -> String {
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

pub(crate) fn get_message(message: *mut MSG) -> BOOL {
    return unsafe { GetMessageA(message, None, 0, 0) };
}

fn get_window_info(handle: HWND, offset: WINDOW_LONG_PTR_INDEX) -> i32 {
    let info = unsafe { GetWindowLongA(handle, offset) };
    return info;
}

pub fn get_dpi(hwnd: HWND) -> u32 {
    return unsafe { GetDpiForWindow(hwnd) };
}

pub fn adjust_for_dpi(rect: &RECT, style: WINDOW_STYLE, dpi: u32) -> RECT {
    let mut adjusted_rect = rect.clone();
    handle_result(unsafe {
        AdjustWindowRectExForDpi(&mut adjusted_rect, style, false, WINDOW_EX_STYLE(0), dpi)
    });
    return adjusted_rect;
}
