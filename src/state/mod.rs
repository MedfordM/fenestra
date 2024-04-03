use windows::Win32::UI::WindowsAndMessaging::HHOOK;

pub mod init;

pub static mut HOOKS: Vec<HHOOK> = Vec::new();