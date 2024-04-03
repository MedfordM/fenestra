use windows::Win32::UI::WindowsAndMessaging::HHOOK;

use crate::util;

pub mod hook_keyboard;
pub fn set_hooks(mut hook_ids: Vec<HHOOK>)  {
  println!("Setting hooks");
  hook_ids.push(util::set_keyboard_hook());
}
  
pub fn unset_hooks(mut hook_ids: Vec<HHOOK>) {
  println!("Unsetting hooks");
    hook_ids.iter().for_each(|hook_id| {
      util::unset_hook(hook_id);
    });
}
