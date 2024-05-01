use log::debug;
use windows::Win32::UI::{Accessibility::HWINEVENTHOOK, WindowsAndMessaging::HHOOK};

use crate::{state::HOOKS, win_api::hook::{unset_event_hook, unset_hook}};

pub mod keyboard;
pub mod window;

pub fn set_hooks() -> Vec<(String, isize)> {
    debug!("Setting hooks");
    let mut hooks: Vec<(String, isize)> = Vec::new();
    hooks.push((String::from("keyboard"), keyboard::init_hook().0));
    hooks.push((String::from("window"), window::init_hook().0));
    return hooks;
}

pub fn unset_hooks() {
    debug!("Unsetting hooks");
    unsafe { HOOKS.iter().for_each(|hook| {
        match hook.0.as_str() {
            "window" => unset_event_hook(HWINEVENTHOOK(hook.1)),
            "keyboard" => unset_hook(HHOOK(hook.1)),
            _ => ()
        };
    }) };
}
