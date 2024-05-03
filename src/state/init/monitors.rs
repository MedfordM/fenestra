use crate::data::common::direction::{Direction, ALL_DIRECTIONS};
use crate::data::monitor::Monitor;
use crate::data::window::Window;
use crate::state::init::workspace;
use crate::win_api::monitor::get_hmonitor_from_window;
use crate::win_api::window;
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::Arc;

pub fn init_windows(monitor: Monitor) -> Vec<Window> {
    let all_windows: HashSet<Window> = window::get_all();
    return all_windows
        .iter()
        .filter(|window| get_hmonitor_from_window(window.hwnd) == monitor.hmonitor)
        .cloned()
        .collect();
}

pub fn init_workspaces(monitors: &Vec<Arc<RefCell<Monitor>>>) {
    monitors.iter().for_each(|monitor_ref| {
        let monitor = monitor_ref.borrow_mut();
        monitor.workspaces = workspace::init_workspaces(monitor_ref);
    });
}

pub fn init_neighbors(monitors: Vec<Monitor>) -> Vec<Arc<RefCell<Monitor>>> {
    let arc_monitors: Vec<Arc<RefCell<Monitor>>> = monitors
        .clone()
        .into_iter()
        .map(|mon| Arc::new(RefCell::new(mon)))
        .collect();
    let monitors: Vec<Monitor> = monitors.clone();
    // let mut monitors = unsafe { &INTERNAL_MONITORS }.clone();
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
    monitors.clone().into_iter().for_each(|monitor| {
        for direction in &ALL_DIRECTIONS {
            let other_monitors: Vec<_> = monitors
                .clone()
                .into_iter()
                .filter(|m| m != &monitor)
                .collect();
            let candidates = other_monitors
                .clone()
                .into_iter()
                .map(|m| m.create_nearest_candidate(direction))
                .collect();
            let max_delta: i32 = match direction {
                Direction::LEFT | Direction::RIGHT => min_width,
                Direction::UP | Direction::DOWN => min_height,
            };
            let nearest_result = direction.find_nearest(
                &monitor.clone().create_nearest_candidate(&direction),
                candidates,
            );
            if nearest_result.is_none() {
                continue;
            }
            let nearest = nearest_result.unwrap();
            if nearest.distance < max_delta {
                continue;
            }
            let nearest_mon = nearest.object;
            let nearest_mon_arc = arc_monitors
                .iter()
                .find(|m| m.borrow().hmonitor == nearest_mon.hmonitor)
                .unwrap();
            let current_mon_arc_ref = arc_monitors
                .iter()
                .find(|m| m.borrow().hmonitor == monitor.hmonitor)
                .unwrap();
            let mut current_mon_arc = current_mon_arc_ref.borrow_mut();
            current_mon_arc
                .neighbors
                .insert(direction.clone(), Arc::clone(nearest_mon_arc));
            // debug!(
            //     "Found neighbor for '{}': '{}'({}) distance {}",
            //     monitor.name, name, direction, nearest_distance
            // );
        }
    });
    return arc_monitors;
}
