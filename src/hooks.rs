use std::process::exit;

use windows::Win32::{Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM}, UI::WindowsAndMessaging::{SetWindowsHookExA, UnhookWindowsHookEx, HHOOK, WH_KEYBOARD_LL}};

pub fn set_hooks() -> HHOOK {
  println!("Setting hooks");
  unsafe { 
    let result = SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_hook_callback), HINSTANCE(0), 0);
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

unsafe extern "system" fn keyboard_hook_callback(code: i32, wparam: WPARAM, _lparam: LPARAM) -> LRESULT {
  if code < 0 {
    return LRESULT::default();
  }

  const WM_KEY_UP: usize = 0x0101;
  const WM_KEY_DOWN: usize = 0x0100;
  const WM_CHAR: usize = 0x0102;

  if code >= 0 {
    match wparam {
      WPARAM(WM_KEY_UP) => {
        println!("Key up");
      }
      WPARAM(WM_KEY_DOWN) => {
        println!("Key down");
      }
      WPARAM(WM_CHAR) => {
        println!("Key pressed {:?}", wparam.0.to_string());
      }
      _ => {}
    }
  }
  return LRESULT::default();
}