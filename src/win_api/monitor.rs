use std::mem;

use log::{debug, error};
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{EnumDisplayMonitors, GetMonitorInfoA, HDC, HMONITOR, MONITOR_DEFAULTTONEAREST, MonitorFromWindow, MONITORINFO, MONITORINFOEXA};

use crate::data::common::direction::{ALL_DIRECTIONS, Direction};
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
        error!("Unable to enumerate displays");
    }

    unsafe {
        let min_width: i32 = MONITORS
            .iter()
            .map(|monitor| {
                let origin: i32 = monitor.position.left.abs();
                let end: i32 = monitor.position.right.abs();
                return (end - origin).abs();
            })
            .min()
            .unwrap();
        let min_height: i32 = MONITORS
            .iter()
            .map(|monitor| {
                let origin: i32 = monitor.position.top.abs();
                let end: i32 = monitor.position.bottom.abs();
                return (end - origin).abs();
            })
            .min()
            .unwrap();
        MONITORS = MONITORS
            .iter()
            .map(|mon| {
                let mut monitor = mon.clone();
                let other_monitors: Vec<Monitor> = MONITORS
                    .iter()
                    .filter(|m| !m.eq(&mon))
                    .map(|m| m.clone())
                    .collect();
                let other_rects: Vec<RECT> = other_monitors.iter().map(|m| m.position).collect();
                for direction in &ALL_DIRECTIONS {
                    let max_delta: i32 = match direction {
                        Direction::LEFT | Direction::RIGHT => min_width,
                        Direction::UP | Direction::DOWN => min_height,
                    };
                    let nearest_result: Option<(RECT, i32)> = direction.find_nearest(
                        monitor.position,
                        String::from(&monitor.name),
                        &other_rects,
                        true,
                        false,
                        Some(max_delta),
                        None
                    );
                    if nearest_result.is_none() {
                        continue;
                    }
                    let (nearest_rect, _): (RECT, _) = nearest_result.unwrap();
                    let nearest_mon = other_monitors
                        .iter()
                        .find(|m| m.position == nearest_rect)
                        .map(|m| m.clone());
                    if nearest_mon.is_some() {
                        let name = nearest_mon.unwrap().name.replace("\0", "");
                        debug!("Returning final match {}", name);
                        monitor.neighbors.push((direction.clone(), name));
                    }
                }
                return monitor.clone();
            })
            .collect();
    };
    return unsafe { MONITORS.clone() };
}

pub fn get_monitor(hmonitor: HMONITOR) -> Monitor {
    let mut monitor_info = MONITORINFOEXA::default();
    monitor_info.monitorInfo.cbSize = mem::size_of::<MONITORINFOEXA>() as u32;
    let monitor_info_ptr = &mut monitor_info as *mut MONITORINFOEXA as *mut MONITORINFO;
    let result = unsafe { GetMonitorInfoA(hmonitor, monitor_info_ptr) };
    if !result.as_bool() {
        error!("Unable to get monitor info");
    }
    // Seriously, there must be a better way
    let name = unsafe {
        std::str::from_utf8_unchecked(&mem::transmute::<[i8; 32], [u8; 32]>(monitor_info.szDevice))
            .replace("\\", "")
            .to_string()
    };
    Monitor {
        hmonitor,
        name,
        position: monitor_info.monitorInfo.rcWork,
        workspaces: Vec::new(),
        neighbors: Vec::new(),
    }
}

pub fn get_monitor_from_window(hwnd: HWND) -> HMONITOR {
    let result = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) };
    if result == HMONITOR::default() {
        error!("Unable to get monitor from window");
    }
    return result;
}
