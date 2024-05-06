use std::process::exit;

use log::error;
use windows::Win32::{Foundation::{HWND, LPARAM, LRESULT, WPARAM}, UI::{Accessibility::{SetWinEventHook, UnhookWinEvent, HWINEVENTHOOK}, WindowsAndMessaging::{CallNextHookEx, SetWindowsHookExA, UnhookWindowsHookEx, EVENT_OBJECT_DESTROY, EVENT_SYSTEM_FOREGROUND, HHOOK, WINDOWS_HOOK_ID, WINEVENT_OUTOFCONTEXT}}};

use super::misc::handle_result;

pub fn set_window_hook(hook_id: WINDOWS_HOOK_ID, callback: unsafe extern "system" fn(i32, WPARAM, LPARAM) -> LRESULT) -> HHOOK {
  return handle_result(unsafe {SetWindowsHookExA(hook_id, Some(callback), None, 0)});
}

pub fn set_event_hook(callback: unsafe extern "system" fn(HWINEVENTHOOK, u32, HWND, i32, i32, u32, u32)) -> HWINEVENTHOOK {
  return unsafe { SetWinEventHook(EVENT_SYSTEM_FOREGROUND, EVENT_OBJECT_DESTROY, None, Some(callback), 0, 0, WINEVENT_OUTOFCONTEXT) };
}

pub fn unset_window_hook(hook: HHOOK) {
  let result = unsafe { UnhookWindowsHookEx(hook) };
  if result.is_err() {
      error!("Failed to unset hook");
      exit(100);
  }
}

pub fn unset_event_hook(hook: HWINEVENTHOOK) {
  let result = unsafe { UnhookWinEvent(hook) };
  if !result.as_bool() {
    error!("Failed to unset event hook");
    exit(100);
  }
}

pub fn call_next_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
  return unsafe { CallNextHookEx(None, code, wparam, lparam) };
}
