use std::f32;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

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
        origin: (String, RECT, Option<u32>, Option<u32>),
        candidates: &Vec<(String, RECT, Option<u32>, Option<u32>)>,
    ) -> Option<(String, i32)> {
        /*
           Virtual Screen Coordinates:
           Lower  X values are in the left direction
           Higher X values are in the right direction
           Lower  Y values are in the up direction
           Higher Y values are in the down direction
        */
        let origin_point: POINT = POINT {
            x: origin.1.left,
            y: origin.1.top,
        };
        let mut nearest: Option<(String, POINT, i32)> = None;
        // debug!(
        //     "Attempting to find nearest({}) candidate from '{}' using offsets {{x: {}, y: {}}}",
        //     self, origin.0, origin_offset_x, origin_offset_y
        // );
        candidates.iter().for_each(|candidate| {
            let candidate_offset_x = candidate.2.unwrap_or_default() as i32;
            let candidate_offset_y = candidate.3.unwrap_or_default() as i32;
            let candidate_point: POINT = POINT {
                x: candidate.1.left,
                y: candidate.1.top,
            };
            // debug!(
            //     "Evaluating {} {:?} with offsets {{x: {}, y: {}}}",
            //     candidate.0, candidate_point, candidate_offset_x, candidate_offset_y
            // );
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
            let distance: f32 = ((delta_x.pow(2) + delta_y.pow(2)) as f32).sqrt();
            // debug!("Calculated '{}' distance as {}", candidate.0, &distance);
            let current_nearest = nearest.clone();
            if current_nearest.is_none() || current_nearest.unwrap().2 > distance as i32 {
                nearest = Some((String::from(&candidate.0), candidate_point, distance as i32));
                // debug!("Current nearest is now {} at {}", candidate.0, distance);
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
