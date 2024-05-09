use log::debug;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use windows::Win32::Foundation::{POINT, RECT};

use crate::data::common::direction::Direction::{DOWN, LEFT, RIGHT, UP};
use crate::data::monitor::Monitor;
use crate::data::window::Window;

#[derive(Clone, PartialEq, Hash, Eq)]
pub enum Direction {
    LEFT,
    DOWN,
    UP,
    RIGHT,
}

pub struct DirectionCandidate {
    pub id: isize,
    pub name: String,
    pub rect: RECT,
    pub offset_x: Option<u32>,
    pub offset_y: Option<u32>,
}

impl From<&Window> for DirectionCandidate {
    fn from(window: &Window) -> DirectionCandidate {
        DirectionCandidate {
            id: window.hwnd.0,
            name: String::from(&window.title),
            rect: RECT {
                left: window.rect.left + window.info.cxWindowBorders as i32,
                top: window.rect.top + window.info.cyWindowBorders as i32,
                right: window.rect.right,
                bottom: window.rect.bottom,
            },
            offset_x: Some(window.info.cxWindowBorders),
            offset_y: Some(window.info.cyWindowBorders),
        }
    }
}

impl From<&Monitor> for DirectionCandidate {
    fn from(monitor: &Monitor) -> DirectionCandidate {
        DirectionCandidate {
            id: monitor.hmonitor.0,
            name: String::from(&monitor.name),
            rect: monitor.info.rcMonitor,
            // rect: match direction {
            //     LEFT | RIGHT => RECT {
            //         left: unsafe { monitor.device_mode.Anonymous1.Anonymous2 }.dmPosition.x,
            //         top: 0,
            //         bottom: 0,
            //         right: 0,
            //     },
            //     UP | DOWN => RECT {
            //         left: 0,
            //         top: unsafe { monitor.device_mode.Anonymous1.Anonymous2 }.dmPosition.y,
            //         bottom: 0,
            //         right: 0,
            //     },
            // },
            offset_x: None,
            offset_y: None,
        }
    }
}

impl DirectionCandidate {}

pub struct DirectionResult {
    pub id: isize,
    pub distance: i32,
    point: POINT,
}

impl Direction {
    pub fn find_nearest(
        &self,
        origin: &DirectionCandidate,
        candidates: Vec<DirectionCandidate>,
    ) -> Option<DirectionResult> {
        /*
           Virtual Screen Coordinates:
           Lower  X values are in the left direction
           Higher X values are in the right direction
           Lower  Y values are in the up direction
           Higher Y values are in the down direction
        */
        let origin_point: POINT = POINT {
            x: origin.rect.left,
            y: origin.rect.top,
        };
        debug!(
            "Attempting to find nearest({}) candidate from '{} at position {:?}'",
            self, origin.name, origin_point
        );
        let mut results: Vec<DirectionResult> = Vec::new();
        candidates.into_iter().for_each(|candidate| {
            let candidate_offset_x = candidate.offset_x.unwrap_or_default() as i32;
            let candidate_offset_y = candidate.offset_y.unwrap_or_default() as i32;
            let candidate_point: POINT = POINT {
                x: candidate.rect.left,
                y: candidate.rect.top,
            };
            debug!(
                "Evaluating candidate '{}' at position {:?}",
                candidate.name, candidate_point
            );
            let delta_x: i32 = candidate_point.x - origin_point.x;
            let delta_y: i32 = candidate_point.y - origin_point.y;
            match &self {
                LEFT => {
                    // Skip if the origin is left of candidate
                    if origin_point.x < candidate_point.x {
                        // debug!("Discarding invalid candidate");
                        return;
                    }
                    if delta_x.abs() <= candidate_offset_x {
                        // debug!("Discarding ambiguous candidate");
                        return;
                    }
                }
                RIGHT => {
                    // Skip if the origin is right of candidate
                    if origin_point.x > candidate_point.x {
                        // debug!("Discarding invalid candidate");
                        return;
                    }
                    if delta_x.abs() <= candidate_offset_x {
                        // debug!("Discarding ambiguous candidate");
                        return;
                    }
                }
                UP => {
                    // Skip if the origin is above candidate
                    if origin_point.y < candidate_point.y {
                        // debug!("Discarding invalid candidate");
                        return;
                    }
                    if delta_y.abs() <= candidate_offset_y {
                        // debug!("Discarding ambiguous candidate");
                        return;
                    }
                }
                DOWN => {
                    // Skip if the origin is below candidate
                    if origin_point.y > candidate_point.y {
                        // debug!("Discarding invalid candidate");
                        return;
                    }
                    if delta_y.abs() <= candidate_offset_y {
                        // debug!("Discarding ambiguous candidate");
                        return;
                    }
                }
            }
            let delta_x_pow = delta_x.pow(2);
            let delta_y_pow = delta_y.pow(2);
            let distance: f64 = ((delta_x_pow + delta_y_pow) as f64).sqrt();
            debug!("Calculated '{}' distance as {}", candidate.name, &distance);
            results.push(DirectionResult {
                id: candidate.id,
                distance: distance as i32,
                point: candidate_point,
            });
        });
        if results.is_empty() {
            return None;
        }
        results.sort_by(|a, b| a.distance.cmp(&b.distance));
        let final_result = results.remove(0);
        return Some(DirectionResult {
            id: final_result.id,
            distance: final_result.distance,
            point: final_result.point,
        });
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_ascii_uppercase().as_str() {
            "LEFT" => Ok(LEFT),
            "DOWN" => Ok(DOWN),
            "UP" => Ok(UP),
            "RIGHT" => Ok(RIGHT),
            _ => Err(()),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            LEFT => String::from("left"),
            DOWN => String::from("down"),
            UP => String::from("up"),
            RIGHT => String::from("right"),
        };
        write!(f, "{}", str)
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub static ALL_DIRECTIONS: [Direction; 4] = [LEFT, DOWN, UP, RIGHT];
