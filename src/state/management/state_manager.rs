use log::warn;
use crate::data::common::state::AppState;
use crate::win_api;
use crate::state::init;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::HMONITOR;
use crate::data::hook::Hook;
use crate::state::management::group_manager::GroupManager;
use crate::state::management::key_manager::KeyManager;
use crate::state::management::monitor_manager::MonitorManager;
use crate::state::management::window_manager::WindowManager;
use crate::state::management::workspace_manager::WorkspaceManager;

pub struct StateManager {
    state: AppState,
    pub window_manager: WindowManager,
    pub group_manager: GroupManager,
    pub workspace_manager: WorkspaceManager,
    pub monitor_manager: MonitorManager,
    pub key_manager: KeyManager
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state: init::application(),
            window_manager: WindowManager::new(Vec::new()),
            group_manager: GroupManager::new(Vec::new()),
            workspace_manager: WorkspaceManager::new(Vec::new()),
            monitor_manager: MonitorManager::new(Vec::new()),
            key_manager: KeyManager::new(init::keybinds())
        }
    }

    pub fn handle(&self) -> HWND {
        self.state.handle.clone()
    }

    pub fn hooks(&mut self) -> &mut Vec<Box<dyn Hook>> {
        &mut self.state.hooks
    }

    pub fn current_monitor(&self) -> HMONITOR {
        self.monitor_manager.get_current()
    }
    
    pub fn current_workspace(&self) -> usize {
        self.workspace_manager.workspace_for_group(self.current_group())
    }
    
    pub fn current_group(&self) -> usize {
        self.group_manager.group_for_hwnd(win_api::window::foreground_hwnd())
    }
    
    pub fn add_window(&mut self, hwnd: HWND) {
        self.group_manager.add_window(self.current_group(), hwnd);
    }
    
    pub fn remove_window(&mut self, hwnd: HWND) {
        self.group_manager.remove_window(hwnd);
    }
    
    pub fn validate(&mut self) {
        let (removed, added) = self.window_manager.validate_windows();
        let mut new_positions = Vec::new();
        removed.into_iter().for_each(|hwnd| {
            new_positions.append(&mut self.group_manager.remove_window(hwnd));  
        });
        // This should never happen, new windows should get added by the event listener
        let group = self.current_group();
        added.into_iter().for_each(|hwnd| {
            warn!("Encountered unmanaged windows during validation, all windows should be added via the event listener");
            new_positions.append(&mut self.group_manager.add_window(group, hwnd));
        });
        new_positions.append(&mut self.group_manager.validate());
        for (hwnd, position) in new_positions {
            self.window_manager.set_position(hwnd, position, 0);
        }
    }
}
