use std::collections::HashSet;
use std::mem;

use log::error;
use windows::core::PCSTR;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{
    EnumDisplayDevicesA, EnumDisplayMonitors, EnumDisplaySettingsA, GetMonitorInfoA,
    MonitorFromWindow, DEVMODEA, DISPLAY_DEVICEA, ENUM_CURRENT_SETTINGS, HDC, HMONITOR,
    MONITORINFO, MONITORINFOEXA, MONITOR_DEFAULTTONEAREST,
};
use windows::Win32::UI::Shell::Common::DEVICE_SCALE_FACTOR;
use windows::Win32::UI::Shell::GetScaleFactorForMonitor;
use windows::Win32::UI::WindowsAndMessaging::EDD_GET_DEVICE_INTERFACE_NAME;

use crate::data::common::direction::{Direction, ALL_DIRECTIONS};
use crate::data::monitor::Monitor;
use crate::data::window::Window;
use crate::win_api::misc::handle_result;
use crate::win_api::window;

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
    return unsafe { get_neighbors(MONITORS.clone()) };
}

pub fn get_monitor(hmonitor: HMONITOR) -> Monitor {
    unsafe {
        let mut monitor_info = MONITORINFOEXA::default();
        monitor_info.monitorInfo.cbSize = mem::size_of::<MONITORINFOEXA>() as u32;
        let monitor_info_ptr = &mut monitor_info as *mut MONITORINFOEXA as *mut MONITORINFO;
        let result = GetMonitorInfoA(hmonitor, monitor_info_ptr);
        if !result.as_bool() {
            error!("Unable to get monitor info");
        }
        let name = String::from_utf8_lossy(monitor_info.szDevice.align_to::<u8>().1)
            .split("\0")
            .next()
            .unwrap()
            .trim_start_matches(r"\")
            .to_string();
        let device_mode = get_device_mode(&name);
        let scale = get_scale(hmonitor);
        Monitor {
            hmonitor,
            name,
            info: monitor_info.monitorInfo,
            device_mode,
            scale,
            workspaces: Monitor::init_workspaces(hmonitor, monitor_info.monitorInfo.rcWork),
            neighbors: Vec::new(),
        }
    }
}

pub fn get_monitor_from_window(hwnd: HWND) -> HMONITOR {
    let result = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) };
    if result == HMONITOR::default() {
        error!("Unable to get monitor from window");
    }
    return result;
}

pub fn get_windows_on_monitor(hmonitor: HMONITOR) -> HashSet<Window> {
    let all_windows: HashSet<Window> = window::get_all();
    return all_windows
        .iter()
        .filter(|window| get_monitor_from_window(window.hwnd) == hmonitor)
        .cloned()
        .collect();
}

// pub fn get_current() -> Monitor {
//     let current_window = get_foreground_handle();
//     let hmonitor = get_monitor_from_window(current_window);
//     return get_monitor(hmonitor);
// }

pub fn get_device_mode(device_name: &str) -> DEVMODEA {
    unsafe {
        let mut display_adapter: DISPLAY_DEVICEA = DISPLAY_DEVICEA {
            cb: u32::try_from(mem::size_of::<DISPLAY_DEVICEA>()).unwrap(),
            ..Default::default()
        };
        let _ = EnumDisplayDevicesA(
            PCSTR::null(),
            0,
            &mut display_adapter,
            EDD_GET_DEVICE_INTERFACE_NAME,
        );
        let mut enum_success = true;
        let mut device_number = 0;
        let mut device_mode: DEVMODEA = DEVMODEA {
            dmSize: u16::try_from(mem::size_of::<DEVMODEA>()).unwrap(),
            ..Default::default()
        };
        while enum_success {
            let mut display: DISPLAY_DEVICEA = DISPLAY_DEVICEA {
                cb: u32::try_from(mem::size_of::<DISPLAY_DEVICEA>()).unwrap(),
                ..Default::default()
            };
            enum_success = EnumDisplayDevicesA(
                PCSTR::null(),
                device_number,
                &mut display,
                EDD_GET_DEVICE_INTERFACE_NAME,
            )
            .as_bool();
            let original_name = String::from_utf8_lossy(display.DeviceName.align_to::<u8>().1);
            let _ = EnumDisplaySettingsA(
                PCSTR(original_name.as_ptr()),
                ENUM_CURRENT_SETTINGS,
                &mut device_mode,
            );
            let name = original_name
                .split("\0")
                .next()
                .unwrap()
                .trim_start_matches(r"\")
                .to_string();
            if name == device_name {
                break;
            }
            device_number = device_number + 1;
        }
        return device_mode;
    };
}

fn get_scale(hmonitor: HMONITOR) -> DEVICE_SCALE_FACTOR {
    return handle_result(unsafe { GetScaleFactorForMonitor(hmonitor) });
}

fn get_neighbors(monitors: Vec<Monitor>) -> Vec<Monitor> {
    let min_width: i32 = monitors
        .iter()
        .map(|monitor| {
            let origin: i32 = monitor.info.rcMonitor.left.abs();
            let end: i32 = monitor.info.rcMonitor.right.abs();
            return (end - origin).abs();
        })
        .max()
        .unwrap();
    let min_height: i32 = monitors
        .iter()
        .map(|monitor| {
            let origin: i32 = monitor.info.rcMonitor.top.abs();
            let end: i32 = monitor.info.rcMonitor.bottom.abs();
            return (end - origin).abs();
        })
        .max()
        .unwrap();
    return monitors
        .iter()
        .map(|mon| unsafe {
            let mut monitor = mon.clone();
            let other_monitors: Vec<Monitor> = monitors
                .iter()
                .filter(|m| !m.eq(&mon))
                .map(|m| m.clone())
                .collect();
            let other_rects: Vec<(String, RECT, Option<u32>, Option<u32>)> = other_monitors
                .iter()
                .map(|m| {
                    (
                        String::from(&m.name),
                        RECT {
                            left: m.device_mode.Anonymous1.Anonymous2.dmPosition.x,
                            top: m.device_mode.Anonymous1.Anonymous2.dmPosition.y,
                            bottom: 0,
                            right: 0,
                        },
                        None,
                        None,
                    )
                })
                .collect();
            for direction in &ALL_DIRECTIONS {
                let max_delta: i32 = match direction {
                    Direction::LEFT | Direction::RIGHT => min_width,
                    Direction::UP | Direction::DOWN => min_height,
                };
                let monitor_rect = match direction {
                    Direction::LEFT | Direction::RIGHT => RECT {
                        left: monitor.device_mode.Anonymous1.Anonymous2.dmPosition.x,
                        top: 0,
                        bottom: 0,
                        right: 0,
                    },
                    Direction::UP | Direction::DOWN => RECT {
                        left: 0,
                        top: monitor.device_mode.Anonymous1.Anonymous2.dmPosition.y,
                        bottom: 0,
                        right: 0,
                    },
                };
                let nearest_result: Option<(String, i32)> = direction.find_nearest(
                    (String::from(&monitor.name), monitor_rect, None, None),
                    &other_rects,
                );
                if nearest_result.is_none() {
                    continue;
                }
                let (nearest_name, nearest_distance): (String, _) = nearest_result.unwrap();
                if nearest_distance < max_delta {
                    continue;
                }
                let nearest_mon = other_monitors
                    .iter()
                    .find(|m| m.name.contains(&nearest_name))
                    .map(|m| m.clone());
                if nearest_mon.is_some() {
                    let nearest_monitor = nearest_mon.unwrap();
                    let name = nearest_monitor.name;
                    if nearest_distance > max_delta {
                        continue;
                    }
                    // debug!(
                    //     "Found neighbor for '{}': '{}'({}) distance {}",
                    //     monitor.name, name, direction, nearest_distance
                    // );
                    monitor.neighbors.push((direction.clone(), name));
                }
            }
            return monitor.clone();
        })
        .collect();
}
