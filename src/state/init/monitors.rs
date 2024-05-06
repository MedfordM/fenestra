use crate::data::common::direction::{Direction, ALL_DIRECTIONS, DirectionCandidate};
use crate::data::monitor::Monitor;
use windows::Win32::Graphics::Gdi::HMONITOR;

pub fn init_neighbors(monitors: Vec<Monitor>) -> Vec<Monitor> {
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
    return monitors.clone().into_iter().map(|mut monitor| {
        let origin = DirectionCandidate::from(&monitor);
        for direction in &ALL_DIRECTIONS {
            let other_monitors: Vec<_> = monitors
                .clone()
                .into_iter()
                .filter(|m| m != &monitor)
                .collect();
            let candidates = other_monitors
                .clone()
                .iter()
                .map(|m| DirectionCandidate::from(m))
                .collect();
            let max_delta: i32 = match direction {
                Direction::LEFT | Direction::RIGHT => min_width,
                Direction::UP | Direction::DOWN => min_height,
            };
            let nearest_result = direction.find_nearest(&origin, candidates);
            if nearest_result.is_none() {
                continue;
            }
            let nearest = nearest_result.unwrap();
            if nearest.distance < max_delta {
                continue;
            }
            let nearest_hmonitor = HMONITOR(nearest.id);
            monitor
                .neighbors
                .insert(direction.clone(), nearest_hmonitor);
            // debug!(
            //     "Found neighbor for '{}': '{}'({}) distance {}",
            //     monitor.name, name, direction, nearest_distance
            // );
        }
        return monitor;
    }).collect();
}
