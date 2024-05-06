use log::debug;
use crate::data::hook::Hook;
use crate::hooks::keyboard::KeyboardHook;
use crate::hooks::window::EventHook;

pub mod keyboard;
pub mod window;

pub fn set_hooks() -> Vec<Box<dyn Hook>> {
    let mut hooks: Vec<Box<dyn Hook>> = vec![Box::new(KeyboardHook::new()), Box::new(EventHook::new())];
    hooks.iter_mut().for_each(|hook| hook.set());
    debug!("Set hooks");
    return hooks;
}

pub fn unset_hooks(hooks: &mut Vec<Box<dyn Hook>>) {
    hooks.iter_mut().for_each(|hook| hook.remove());
    debug!("Unset hooks");
}
