use std::collections::HashMap;
use std::mem;

use log::error;
use windows::core::PCSTR;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM, RECT};
use windows::Win32::Graphics::Gdi::{
    EnumDisplayDevicesA, EnumDisplayMonitors, EnumDisplaySettingsA, GetMonitorInfoA,
    MonitorFromWindow, DEVMODEA, DISPLAY_DEVICEA, ENUM_CURRENT_SETTINGS, HDC, HMONITOR,
    MONITORINFO, MONITORINFOEXA, MONITOR_DEFAULTTONEAREST,
};
use windows::Win32::UI::HiDpi::{GetDpiForMonitor, MDT_EFFECTIVE_DPI};
use windows::Win32::UI::Shell::Common::DEVICE_SCALE_FACTOR;
use windows::Win32::UI::Shell::GetScaleFactorForMonitor;
use windows::Win32::UI::WindowsAndMessaging::EDD_GET_DEVICE_INTERFACE_NAME;

use crate::data::monitor::Monitor;
use crate::win_api::misc::handle_result;

static mut INTERNAL_MONITORS: Vec<Monitor> = Vec::new();
pub fn get_all() -> Vec<Monitor> {
    extern "system" fn enum_displays_callback(
        hmonitor: HMONITOR,
        _hdc: HDC,
        _rect: *mut RECT,
        _lparam: LPARAM,
    ) -> BOOL {
        let monitor: Monitor = get_monitor(hmonitor);
        unsafe { INTERNAL_MONITORS.push(monitor) };
        return BOOL::from(true);
    }
    let result = unsafe { EnumDisplayMonitors(None, None, Some(enum_displays_callback), None) };
    if !result.as_bool() {
        error!("Unable to enumerate displays");
    }
    return unsafe { INTERNAL_MONITORS.clone() };
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
        let dpi = get_dpi(hmonitor);
        Monitor {
            hmonitor,
            name,
            info: monitor_info.monitorInfo,
            device_mode,
            scale,
            dpi,
            neighbors: HashMap::new(),
            workspaces: Vec::new(),
        }
    }
}

pub fn hmonitor_from_hwnd(hwnd: HWND) -> HMONITOR {
    let result = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST) };
    if result == HMONITOR::default() {
        error!("Unable to get monitor from window");
    }
    return result;
}

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

fn get_dpi(hmonitor: HMONITOR) -> u32 {
    let mut dpi = 96;
    handle_result(unsafe { GetDpiForMonitor(hmonitor, MDT_EFFECTIVE_DPI, &mut dpi, &mut dpi) });
    return dpi;
}
