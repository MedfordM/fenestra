use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use log::debug;
use windows::Win32::Foundation::RECT;

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
        origin: RECT,
        origin_name: String,
        candidates: &Vec<RECT>,
        discard_overlapping: bool,
        require_non_zero_delta: bool,
        largest_delta: Option<i32>,
        offset_opt: Option<i32>
    ) -> Option<(RECT, i32)> {
        // TODO: This is super broken, refactor to instead compare two POINTs (left,top) on a coordinate plane
        let offset = offset_opt.unwrap_or_default();
        let mut nearest: Option<(RECT, i32)> = None;
        debug!("Attempting to find nearest candidate {:?} from {}", self, origin_name);
        candidates.iter().for_each(|candidate| {
            // Skip evaluation if candidate rect is in the same place as the origin
            if candidate == &origin {
                return;
            }
            let mut origin_coord: i32 = 0 + offset; // origin rect
            let mut candidate_coord: i32 = 0 - offset; // rect being evaluated
            debug!("Evaluating candidate {:?} with offset {}", candidate, offset);
            match &self {
                LEFT => {
                    if origin.left <= candidate.left {
                        return;
                    }
                    if discard_overlapping && origin.right == candidate.right {
                        debug!("Discarding candidate: overlaps with origin");
                        return;
                    }
                    origin_coord += origin.left;
                    candidate_coord += candidate.right;
                }
                RIGHT => {
                    if origin.right >= candidate.right {
                        return;
                    }
                    if discard_overlapping && origin.left == candidate.left {
                        debug!("Discarding candidate: overlaps with origin");
                        return;
                    }
                    origin_coord += origin.right;
                    candidate_coord += candidate.left;
                }
                UP => {
                    if origin.top <= candidate.top {
                        return;
                    }
                    if discard_overlapping && origin.left == candidate.left {
                        debug!("Discarding candidate: overlaps with origin");
                        return;
                    }
                    origin_coord += origin.top;
                    candidate_coord += candidate.bottom;
                }
                DOWN => {
                    if origin.bottom >= candidate.bottom {
                        return;
                    }
                    if discard_overlapping && origin.left == candidate.left {
                        debug!("Discarding candidate: overlaps with origin");
                        return;
                    }
                    // Prioritize locality
                    // let delta_left: i32 = (candidate.left - origin.left).abs();
                    // let delta_right: i32 = (candidate.right - origin.right).abs();
                    // if delta_left == 0 && delta_right == 0 {
                    //     debug!("Prioritizing local candidate");
                    //     origin_coord = 1;
                    //     candidate_coord = 2;
                    // } else if offset.is_some() && delta_left <= offset.unwrap() && delta_right <= offset.unwrap() {
                    //     debug!("Prioritizing local candidate");
                    //     origin_coord = 1;
                    //     candidate_coord = 2;
                    // } else {
                        origin_coord += origin.bottom;
                        candidate_coord += candidate.top;
                    // }
                }
            }
            let delta: i32 = candidate_coord - origin_coord;
            debug!("Calculated candidate delta as {}", delta);
            if require_non_zero_delta && delta == 0 {
                debug!("Discarding candidate: delta=0");
                return;
            }
            if largest_delta.is_some() && largest_delta.unwrap() < delta {
                debug!(
                    "Discarding candidate: delta({})>max_delta({})",
                    delta,
                    largest_delta.unwrap()
                );
                return;
            }
            if nearest.is_none() || delta < nearest.unwrap().1 {
                nearest = Some((candidate.clone(), delta));
            }
        });
        if nearest.is_none() {
            return None;
        }
        return Some(nearest.unwrap());
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
