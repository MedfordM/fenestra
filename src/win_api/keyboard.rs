use std::ffi::CString;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyNameTextA, MapVirtualKeyA, MAPVK_VK_TO_VSC, VkKeyScanA,
};
use windows::Win32::UI::WindowsAndMessaging::{HHOOK, SetWindowsHookExA, WH_KEYBOARD_LL};

use crate::data::key::{KEY_ALT, KEY_CONTROL, KEY_SHIFT, KEY_SPACE, KEY_WINDOWS};
use crate::hooks::hook_keyboard::keyboard_hook::callback;
use crate::win_api::misc::handle_result;
use crate::win_api::window::get_handle;

pub fn set_keyboard_hook() -> HHOOK {
    return handle_result(unsafe {
        SetWindowsHookExA(WH_KEYBOARD_LL, Some(callback), get_handle(), 0)
    });
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
    let result = match key {
        "SPACE" => KEY_SPACE,
        "WIN" => KEY_WINDOWS,
        "CTRL" => KEY_CONTROL,
        "ALT" => KEY_ALT,
        "SHIFT" => KEY_SHIFT,
        _ => unsafe { VkKeyScanA(key.as_bytes()[0] as i8) as i32 },
    };

    if result == 0 {
        println!("Failed to fetch key code for character {}", key);
    }

    return result;
}