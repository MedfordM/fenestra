use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::{WINDOWINFO, WINDOWPLACEMENT};

#[derive(Debug, Clone)]
pub struct Window {
    pub title: String,
    pub hwnd: HWND,
    pub thread_id: u32,
    pub process_id: u32,
    pub rect: RECT,
    pub bounding_rect: RECT,
    pub border_thickness: u32,
    pub info: WINDOWINFO,
    pub placement: WINDOWPLACEMENT,
    pub dpi: u32,
    pub style: i32,
    pub extended_style: i32,
    pub focused: bool
}

impl Eq for Window {}

impl PartialEq for Window {
    fn eq(&self, other: &Self) -> bool {
        self.hwnd == other.hwnd
            || self.title == other.title
            || self.thread_id == other.thread_id
            || self.process_id == other.process_id
    }
}
