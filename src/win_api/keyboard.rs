use std::ffi::CString;

use log::error;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyNameTextA, MapVirtualKeyA, VkKeyScanA, MAPVK_VK_TO_VSC,
};

use crate::data::key::{KEY_ALT, KEY_CONTROL, KEY_SHIFT, KEY_SPACE, WINDOWS_KEY_CODE};

pub fn get_key_name(key_code: i32) -> String {
    let scan_code = unsafe { MapVirtualKeyA(key_code as u32, MAPVK_VK_TO_VSC) };
    let mut buffer = vec![0; 32];
    let result = unsafe { GetKeyNameTextA((scan_code << 16) as i32, &mut buffer) };

    if result == 0 {
        error!("Failed to fetch key name for code {}", key_code);
        return String::new();
    }
    unsafe { buffer.set_len(result as usize) }
    return CString::new(buffer).unwrap().into_string().unwrap();
}

pub fn get_key_code(key: &str) -> i32 {
    let result = match key.to_uppercase().as_str() {
        "SPACE" => KEY_SPACE,
        "WIN" => WINDOWS_KEY_CODE,
        "CTRL" => KEY_CONTROL,
        "ALT" => KEY_ALT,
        "SHIFT" => KEY_SHIFT,
        _ => unsafe { VkKeyScanA(key.as_bytes()[0] as i8) as i32 },
    };

    if result == 0 {
        error!("Failed to fetch key code for character {}", key);
    }

    return result;
}
