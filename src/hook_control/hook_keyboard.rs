
pub mod keyboard_hook {
  use windows::Win32::{Foundation::{LPARAM, LRESULT, WPARAM}, UI::WindowsAndMessaging::KBDLLHOOKSTRUCT};
  pub unsafe extern "system" fn callback(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code < 0 {
      return LRESULT::default();
    }
            
    const WM_KEY_UP: usize = 0x0101;
    const WM_KEY_DOWN: usize = 0x0100;
    const KEY_SPACE: u32 = 0x20;
    const KEY_WINDOWS: u32 = 0x5B;

    if code >= 0 {
      let hook_struct = lparam.0 as *mut KBDLLHOOKSTRUCT;
      let key_code = hook_struct.as_mut().unwrap().vkCode;
      match wparam {
        WPARAM(WM_KEY_UP) => {
          if (key_code) == KEY_SPACE {
            println!("Space key up");
          } else if (key_code) == KEY_WINDOWS {
            println!("Windows key up");
          }
        }
        WPARAM(WM_KEY_DOWN) => {  
          if (key_code) == KEY_SPACE {
            println!("Space key down");
          } else if (key_code) == KEY_WINDOWS {
            println!("Windows key down");
          }
        }
        _ => {}
      }
    }
    return LRESULT::default();
  }
}