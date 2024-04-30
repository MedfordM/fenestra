use std::f32;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use windows::Win32::Foundation::{POINT, RECT};

use crate::data::common::direction::Direction::{DOWN, LEFT, RIGHT, UP};

#[derive(Clone, PartialEq, Hash, Eq)]
pub enum Direction {
    LEFT,
    DOWN,
    UP,
    RIGHT,
}

pub struct DirectionCandidate<'a, T> {
    pub object: &'a T,
    pub name: String,
    pub rect: RECT,
    pub offset_x: Option<u32>,
    pub offset_y: Option<u32>,
}

pub struct DirectionResult<'a, T> {
    pub object: &'a T,
    pub distance: i32,
    point: POINT,
}

impl Direction {
    pub fn find_nearest<'a, 'b, T>(
        &'a self,
        origin: &'b DirectionCandidate<T>,
        candidates: &'b Vec<DirectionCandidate<'a, T>>,
    ) -> Option<DirectionResult<T>> {
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
        // debug!(
        //     "Attempting to find nearest({}) candidate from '{}' using offsets {{x: {}, y: {}}}",
        //     self, origin.0, origin_offset_x, origin_offset_y
        // );
        let mut results: Vec<DirectionResult<T>> = Vec::new();
        candidates.iter().for_each(|candidate| {
            let candidate_offset_x = candidate.offset_x.unwrap_or_default() as i32;
            let candidate_offset_y = candidate.offset_y.unwrap_or_default() as i32;
            let candidate_point: POINT = POINT {
                x: candidate.rect.left,
                y: candidate.rect.top,
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
            results.push(DirectionResult {
                object: &candidate.object,
                distance: distance as i32,
                point: candidate_point,
            });
        });
        if results.is_empty() {
            return None;
        }
        results.sort_by(|a, b| a.distance.cmp(&b.distance));
        let final_result = &results[0];
        return Some(DirectionResult {
            object: final_result.object,
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
