use crate::data::common::axis::Axis;
use windows::Win32::Foundation::{HWND, RECT};

#[derive(Debug)]
pub struct Group {
    pub index: usize,
    pub split_axis: Axis,
    pub rect: RECT,
    pub windows: Vec<HWND>,
}

impl Group {
    pub fn new(index: usize, split_axis: Axis, rect: RECT, windows: Vec<HWND>) -> Self {
        Self { index, split_axis, rect, windows }
    }
}