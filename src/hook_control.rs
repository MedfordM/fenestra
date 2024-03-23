use std::process::exit;

use windows::Win32::{Foundation::HINSTANCE, UI::WindowsAndMessaging::{SetWindowsHookExA, UnhookWindowsHookEx, HHOOK, WH_KEYBOARD_LL}};

use crate::hook_control::hook_keyboard::keyboard_hook::callback;


pub mod hook_keyboard;

pub fn set_hooks() -> HHOOK {
  println!("Setting hooks");
  unsafe { 
    let result = SetWindowsHookExA(WH_KEYBOARD_LL, Some(callback), HINSTANCE(0), 0);
    if result.is_err() {
      println!("Error setting hooks");
      exit(100);
    } else {
      return result.unwrap();
    }
  };
}
  
pub fn unset_hooks(hook_id: HHOOK) {
  println!("Unsetting hooks");
  unsafe {
    let result = UnhookWindowsHookEx(hook_id);
    if result.is_err() {
      println!("Error unsetting hooks");
    }
  }
}
