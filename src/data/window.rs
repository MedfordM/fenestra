use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::WINDOWPLACEMENT;

#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    pub title: String,
    pub hwnd: HWND,
    pub thread_id: u32,
    pub process_id: u32,
    pub placement: WINDOWPLACEMENT,
}
