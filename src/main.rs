use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::System::LibraryLoader::GetModuleHandleA, Win32::UI::WindowsAndMessaging::*,
};
mod hook_control;
mod data;
pub mod window_control;

const WINDOW_CLASS_NAME: PCSTR = PCSTR("WindowManager\0".as_ptr());
const WINDOW_NAME: PCSTR = PCSTR("Rust window\0".as_ptr());


fn main() -> Result<()> {
    // let hook_id = hook_control::set_hooks();
    let app_instance_result = unsafe { GetModuleHandleA(None) };

    if app_instance_result.is_err() {
        println!("Error getting application handle");
        let error: WIN32_ERROR = unsafe { GetLastError() };
        println!("Error code: {:?}", error);
    }

    let app_instance = app_instance_result.unwrap();

    let window_class = WNDCLASSA {
        style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(window_callback),
        hInstance: HINSTANCE(app_instance.0),
        hIcon: HICON::default(),
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW)? },
        lpszClassName: WINDOW_CLASS_NAME,
        ..Default::default()
    };

    if unsafe { RegisterClassA(&window_class) } == 0 {
        println!("Error registering window class");
        let error: WIN32_ERROR = unsafe { GetLastError() };
        println!("Error code: {:?}", error);
    }

    let window_handle = unsafe {
        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            WINDOW_CLASS_NAME,
            WINDOW_CLASS_NAME,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            app_instance,
            None
        )
    };

    let mut message: MSG = MSG::default();
    while unsafe { GetMessageA(&mut message, window_handle, 0, 0).into() } {
        unsafe { TranslateMessage(&message); }
        unsafe { DispatchMessageA(&message); }
        if message.message == WM_NULL {
            break;
        }
    };
    Ok(())
}

extern "system" fn window_callback(window: HWND, message: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match message {
        WM_PAINT =>  unsafe {
            println!("WM_PAINT");
            _ = ValidateRect(window, None);
            LRESULT(0)
        }
        WM_DESTROY => unsafe {
            println!("WM_DESTROY");
            // hook_control::unset_hooks(hook_id);
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcA(window, message, w_param, l_param) }
    }
}