use log::debug;
use windows::Win32::UI::{Accessibility::HWINEVENTHOOK, WindowsAndMessaging::HHOOK};

use crate::win_api::hook::{unset_event_hook, unset_hook};

pub mod keyboard;
pub mod window;

pub fn set_hooks() -> Vec<(String, isize)> {
    let mut hooks: Vec<(String, isize)> = Vec::new();
    hooks.push((String::from("keyboard"), keyboard::init_hook().0));
    hooks.push((String::from("window"), window::init_hook().0));
    debug!("Set hooks");
    return hooks;
}

pub fn unset_hooks(hooks: &Vec<(String, isize)>) {
    unsafe {
        hooks.iter().for_each(|hook| {
            match hook.0.as_str() {
                "window" => unset_event_hook(HWINEVENTHOOK(hook.1)),
                "keyboard" => unset_hook(HHOOK(hook.1)),
                _ => (),
            };
        })
    };
    debug!("Unset hooks");
}
