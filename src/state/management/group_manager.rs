use windows::Win32::Foundation::{HWND, RECT};
use crate::data::common::axis::Axis;
use crate::data::group::Group;

pub struct GroupManager {
    groups: Vec<Group>
}

impl GroupManager {
    pub fn new(groups: Vec<Group>) -> Self {
        Self { groups }
    }
    
    pub fn group_for_hwnd(&self, hwnd: HWND) -> usize {
        self.groups
            .iter()
            .position(|group| group.windows.contains(&hwnd))
            .expect("Unable to fetch group for the requested hwnd")
    }

    pub fn add_window(&mut self, group_index: usize, hwnd: HWND) -> Vec<(HWND, RECT)> {
        self.groups[group_index].windows.push(hwnd);
        return self.calculate_window_positions(vec![group_index]);
    }

    pub fn remove_window(&mut self, hwnd: HWND) -> Vec<(HWND, RECT)> {
        let group_index = self.get_group_index_by_hwnd(hwnd);
        let group= self.get_group(group_index);
        group.windows.retain(|h|h != &hwnd);
        return self.calculate_window_positions(vec![group_index]);
    }

    pub fn swap_windows(&mut self, hwnd_1: HWND, hwnd_2: HWND) -> Vec<(HWND, RECT)> {
        let group_index_1 = self.get_group_index_by_hwnd(hwnd_1);
        let group_index_2 = self.get_group_index_by_hwnd(hwnd_2);
        let window_index_1 = self.get_window_index_in_group(group_index_1, hwnd_1);
        let window_index_2 = self.get_window_index_in_group(group_index_2, hwnd_2);
        let mut window_1 = &self.groups[group_index_1].windows[window_index_1];
        let mut window_2 = &self.groups[group_index_2].windows[window_index_2];
        std::mem::swap(&mut window_1, &mut window_2);
        return self.calculate_window_positions(vec![group_index_1, group_index_2]);
    }
    
    pub fn hwnds_from_groups(&self, group_ids: &Vec<usize>) -> Vec<HWND> {
        let mut hwnds = Vec::new();
        for group_id in group_ids {
            hwnds.extend(&self.groups[*group_id].windows);
        }
        return hwnds;
    }
    
    pub fn calculate_window_positions(&self, group_ids: Vec<usize>) -> Vec<(HWND, RECT)> {
        let mut window_positions = Vec::new();
        let num_groups = self.groups.len();
        for group_id in group_ids {
            let group = &self.groups[group_id];
            let rect_width = group.rect.right - group.rect.left;
            let rect_height = group.rect.bottom - group.rect.top;
            let group_width = rect_width as f32 / num_groups as f32;
            let windows = &group.windows;
            let num_windows = windows.len();
            let (section_width, section_height) = match group.split_axis {
                Axis::HORIZONTAL => (group_width as i32, (rect_height as f32 / num_windows as f32) as i32),
                Axis::VERTICAL => ((group_width / num_windows as f32) as i32, (rect_height as f32) as i32)
            };
            for window_index in 0..num_windows {
                let new_position = match group.split_axis {
                    Axis::HORIZONTAL => {
                        let top = group.rect.top + (section_height * window_index as i32);
                        RECT {
                            top,
                            bottom: top + section_height,
                            ..group.rect
                        }
                    }
                    Axis::VERTICAL => {
                        let left = group.rect.left + (section_width * window_index as i32);
                        RECT {
                            left,
                            right: left + section_width,
                            ..group.rect
                        }
                    }
                };
                window_positions.push((windows[window_index], new_position));
            }
        }
        return window_positions;
    }
    
    // Validate each hwnd only exists in one group
    pub fn validate(&mut self) -> Vec<(HWND, RECT)> {
        let mut all_hwnds = Vec::new();
        let mut updated_groups = Vec::new();
        self.groups.iter_mut().for_each(|group| {
            let before_len = group.windows.len();
            group.windows.retain(|hwnd| !all_hwnds.contains(&hwnd));
            let after_len = group.windows.len();
            if before_len != after_len {
                updated_groups.push(group.index);
            }
            all_hwnds.extend(group.windows.iter());
        });
        return self.calculate_window_positions(updated_groups);
    }

    fn get_group(&mut self, index: usize) -> &mut Group {
        self.groups
            .get_mut(index)
            .expect("Unable to fetch group for the requested index")
    }
    
    fn hwnds_from_group(&self, group_id: usize) -> &Vec<HWND> {
        &self.groups[group_id].windows
    }


    fn get_group_index_by_hwnd(&self, hwnd: HWND) -> usize {
        self.groups
            .iter()
            .position(|group| group.windows.contains(&hwnd))
            .expect("Unable to fetch group for the requested hwnd")
    }

    fn get_window_index_in_group(&self, group_index: usize, hwnd: HWND) -> usize {
        self.groups[group_index].windows
            .iter()
            .position(|h| h == &hwnd)
            .expect("Unable to fetch hwnd index within group")
    }
}