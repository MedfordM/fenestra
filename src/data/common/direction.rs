use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use log::debug;
use windows::Win32::Foundation::{POINT, RECT};

use crate::data::common::direction::Direction::{DOWN, LEFT, RIGHT, UP};

#[derive(Clone, PartialEq)]
pub enum Direction {
    LEFT,
    DOWN,
    UP,
    RIGHT,
}

impl Direction {
    pub fn find_nearest(
        &self,
        origin_rect: RECT,
        origin_name: String,
        candidates: &Vec<(String, RECT)>,
        discard_overlapping: bool,
        require_non_zero_delta: bool,
        largest_delta: Option<i32>,
        offset_opt: Option<i32>,
    ) -> Option<(RECT, i32)> {
        /*
           Virtual Screen Coordinates:
           Lower  X values are in the left direction
           Higher X values are in the right direction
           Lower  Y values are in the up direction
           Higher Y values are in the down direction
        */
        let offset = offset_opt.unwrap_or_default();
        let origin_point: POINT = POINT {
            x: origin_rect.left,
            y: origin_rect.top,
        };
        let mut nearest: Option<(RECT, POINT, i32)> = None;
        debug!(
            "Attempting to find nearest candidate {:?} from {}",
            self, origin_name
        );
        candidates.iter().for_each(|candidate| {
            let candidate_point: POINT = POINT {
                x: candidate.1.left,
                y: candidate.1.top,
            };
            debug!(
                "Evaluating {} {:?} with offset {}",
                candidate.0, candidate_point, offset
            );
            match &self {
                LEFT => {
                    // Skip if nearest point is left of candidate
                    if nearest.is_some() && nearest.unwrap().1.x < candidate_point.x {
                        return;
                    }
                    // Skip if the origin is left of candidate
                    if origin_point.x < candidate_point.x {
                        return;
                    }
                }
                RIGHT => {
                    // Skip if nearest point is right of candidate
                    if nearest.is_some() && nearest.unwrap().1.x > candidate_point.x {
                        return;
                    }
                    // Skip if the origin is right of candidate
                    if origin_point.x > candidate_point.x {
                        return;
                    }
                }
                UP => {
                    // Skip if nearest point is above candidate
                    if nearest.is_some() && nearest.unwrap().1.y < candidate_point.y {
                        return;
                    }
                    // Skip if the origin is above candidate
                    if origin_point.y < candidate_point.y {
                        return;
                    }
                }
                DOWN => {
                    // Skip if nearest point is below candidate
                    if nearest.is_some() && nearest.unwrap().1.y > candidate_point.y {
                        return;
                    }
                    // Skip if the origin is below candidate
                    if origin_point.y > candidate_point.y {
                        return;
                    }
                }
            }
            let delta_x: i32 = origin_point.x - candidate_point.x;
            let delta_y: i32 = origin_point.y - candidate_point.y;
            let delta: i32 = (delta_x * delta_x) + (delta_y * delta_y);
            debug!("Calculated {} delta as {}", candidate.0, delta);
            if nearest.is_none() || nearest.unwrap().2 > delta {
                nearest = Some((candidate.1, candidate_point, delta));
            }
        });
        if nearest.is_none() {
            return None;
        }
        let nearest_result = nearest.unwrap();
        return Some((nearest_result.0, nearest_result.2));
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
