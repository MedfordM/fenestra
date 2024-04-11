use std::mem;

use windows::Win32::Foundation::{BOOL, LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{
    EnumDisplayMonitors, GetMonitorInfoA, HDC, HMONITOR, MONITORINFO, MONITORINFOEXA,
};

use crate::data::monitor::Monitor;

static mut MONITORS: Vec<Monitor> = Vec::new();
pub fn get_all() -> Vec<Monitor> {
    extern "system" fn enum_displays_callback(
        hmonitor: HMONITOR,
        _hdc: HDC,
        _rect: *mut RECT,
        _lparam: LPARAM,
    ) -> BOOL {
        let monitor: Monitor = Monitor::from(hmonitor);
        unsafe {
            MONITORS.push(monitor);
        }
        return BOOL::from(true);
    }
    let result = unsafe { EnumDisplayMonitors(None, None, Some(enum_displays_callback), None) };
    if !result.as_bool() {
        println!("Unable to enumerate displays");
    }
    return unsafe { MONITORS.clone() };
}

pub fn get_monitor(hmonitor: HMONITOR) -> Monitor {
    println!("test {:?}", hmonitor);
    let mut monitor_info = MONITORINFOEXA::default();
    monitor_info.monitorInfo.cbSize = mem::size_of::<MONITORINFOEXA>() as u32;
    let monitor_info_ptr = &mut monitor_info as *mut MONITORINFOEXA as *mut MONITORINFO;
    let result = unsafe { GetMonitorInfoA(hmonitor, monitor_info_ptr) };
    if !result.as_bool() {
        println!("Unable to get monitor info");
    }
    // Seriously, there must be a better way
    let name = unsafe {
        std::str::from_utf8_unchecked(&mem::transmute::<[i8; 32], [u8; 32]>(monitor_info.szDevice))
            .to_string()
    };
    Monitor {
        name,
        position: monitor_info.monitorInfo.rcWork,
        workspaces: Vec::new(),
    }
}
