use windows::Win32::UI::WindowsAndMessaging::HHOOK;

use crate::win_api;

pub mod hook_keyboard;

pub fn set_hooks() -> Vec<HHOOK> {
    println!("Setting hooks");
    let mut hooks: Vec<HHOOK> = Vec::new();
    hooks.push(win_api::keyboard::set_keyboard_hook());
    return hooks;
}

pub fn unset_hooks(hook_ids: &Vec<HHOOK>) {
    println!("Unsetting hooks");
    hook_ids.iter().for_each(|hook_id| {
        win_api::misc::unset_hook(hook_id);
    });
}
