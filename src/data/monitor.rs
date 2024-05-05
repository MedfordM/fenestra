use std::collections::HashMap;

use windows::Win32::Graphics::Gdi::{DEVMODEA, HMONITOR, MONITORINFO};
use windows::Win32::UI::Shell::Common::DEVICE_SCALE_FACTOR;

use crate::data::common::direction::Direction;

#[derive(Clone)]
pub struct Monitor {
    pub hmonitor: HMONITOR,
    pub name: String,
    pub info: MONITORINFO,
    pub device_mode: DEVMODEA,
    pub scale: DEVICE_SCALE_FACTOR,
    pub neighbors: HashMap<Direction, Vec<HMONITOR>>,
    pub workspaces: Vec<usize>,
    pub focused: bool
}

impl Monitor {
    // pub fn current_workspace(&mut self) -> &mut Workspace {
    //     return self
    //         .workspaces
    //         .iter_mut()
    //         .find(|workspace| workspace.focused == true)
    //         .expect("Unable to find current workspace");
    // }

    // pub fn get_workspace(&mut self, id: u32) -> &mut Workspace {
    //     return &mut self.workspaces[(id - 1) as usize];
    // }

    // pub fn focus_workspace(&mut self, id: u32) {
    //     let current_workspace = self.current_workspace();
    //     if id == current_workspace.id {
    //         debug!("Skipping request to focus the current workspace");
    //         return;
    //     }
    //     current_workspace.unfocus();
    //     self.workspaces[(id - 1) as usize].focus();
    // }

    // pub fn add_window_to_workspace(&mut self, id: u32, window: Window) {
    //     let current_workspace = self.current_workspace();
    //     if id == current_workspace.id {
    //         return;
    //     }
    //     current_workspace.remove_window(&window);
    //     self.workspaces[(id - 1) as usize].add_window(window);
    // }

    // fn workspace_from_hwnd(&mut self, hwnd: &HWND) -> Option<&mut Workspace> {
    //     if !self.contains_hwnd(hwnd) {
    //         return None;
    //     }
    //     return self
    //         .workspaces
    //         .iter_mut()
    //         .find(|workspace| workspace.contains_hwnd(hwnd));
    // }

    // pub fn workspace_from_window(&mut self, window: &Window) -> Option<&mut Workspace> {
    //     return self.workspace_from_hwnd(&window.hwnd);
    // }

    // pub fn init_workspaces(&self) -> Vec<Workspace> {
    //     return workspaces;
    // }

    // pub fn contains_window(&self, window: &Window) -> bool {
    //     return self.contains_hwnd(&window.hwnd);
    // }

    // pub fn contains_hwnd(&self, hwnd: &HWND) -> bool {
    //     return self.all_windows().iter().any(|window| window.hwnd == *hwnd);
    // }

    // pub fn add_window(&mut self, window: Window) -> bool {
    //     let current_workspace = self.current_workspace();
    //     return current_workspace.add_window(window);
    // }

    // pub fn remove_hwnd(&mut self, hwnd: &HWND) -> bool {
    //     if !self.contains_hwnd(hwnd) {
    //         return false;
    //     }
    //     let workspace = self.workspace_from_hwnd(hwnd);
    //     if workspace.is_none() {
    //         return false;
    //     }
    //     return workspace.unwrap().remove_hwnd(hwnd);
    // }

    // pub fn remove_window(&mut self, window: &Window) -> bool {
    //     if !self.contains_window(window) {
    //         return false;
    //     }
    //     let workspace = self.workspace_from_hwnd(&window.hwnd);
    //     if workspace.is_none() {
    //         return false;
    //     }
    //     return workspace.unwrap().remove_window(window);
    // }

    // pub fn all_windows(&self) -> HashSet<Window> {
    //     let mut all_windows: HashSet<Window> = HashSet::new();
    //     self.workspaces.iter().for_each(|workspace| {
    //         all_windows.extend(workspace.all_windows().clone());
    //     });
    //     return all_windows;
    // }

    // pub fn window_from_hwnd(&mut self, hwnd: &HWND) -> Option<&mut Window> {
    //     if !self.contains_hwnd(hwnd) {
    //         return None;
    //     }
    //     let workspace = self.workspace_from_hwnd(hwnd).unwrap();
    //     let group = workspace.group_from_hwnd(hwnd).unwrap();
    //     return group.get_window(hwnd);
    // }

    // pub fn move_window_in_direction(&mut self, hwnd: &HWND, direction: &Direction) {
    //     let workspace_result = self.workspace_from_hwnd(hwnd);
    //     if workspace_result.is_none() {
    //         return;
    //     }
    //     let workspace = workspace_result.unwrap();
    //     let window_result = workspace.window_from_hwnd(hwnd);
    //     if window_result.is_none() {
    //         return;
    //     }
    //     let window = window_result.unwrap();
    //     window.move_in_direction(direction);
    //     workspace.arrange_windows();
    // }
}

impl PartialEq for Monitor {
    fn eq(&self, other: &Self) -> bool {
        self.hmonitor == other.hmonitor
    }
}