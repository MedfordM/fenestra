use windows::Win32::UI::WindowsAndMessaging::HHOOK;

use crate::util;

pub mod hook_keyboard;

pub fn set_hooks() -> Vec<HHOOK>  {
  println!("Setting hooks");
  let mut hooks: Vec<HHOOK> = Vec::new();
  //hooks.push(util::set_keyboard_hook());
  return hooks;
}
  
pub fn unset_hooks(hook_ids: Vec<HHOOK>) {
  println!("Unsetting hooks");
  hook_ids.iter().for_each(|hook_id| {
    util::unset_hook(hook_id);
  });
}
