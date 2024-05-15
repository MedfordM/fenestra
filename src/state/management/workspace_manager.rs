use crate::data::common::direction::Direction;
use crate::data::workspace::Workspace;

pub struct WorkspaceManager {
    workspaces: Vec<Workspace>,
}

impl WorkspaceManager {
    pub fn new(workspaces: Vec<Workspace>) -> Self {
        Self { workspaces }
    }

    pub fn all(&self) -> Vec<usize> {
        self.workspaces
            .iter()
            .map(|workspace| workspace.index)
            .collect()
    }

    pub fn group_in_direction(&self, group: usize, direction: &Direction) -> Option<usize> {
        let workspace_index = self.workspace_for_group(group);
        let groups = self.groups_for_workspace(workspace_index);
        direction.adjacent_item(group, groups)
    }

    pub fn active_workspace(&self, workspace_ids: &Vec<usize>) -> usize {
        self.workspaces
            .iter()
            .position(|workspace| workspace_ids.contains(&workspace.index) && workspace.active)
            .expect("Unable to fetch active workspace for requested workspace ids")
    }

    pub fn workspace_for_group(&self, group_id: usize) -> usize {
        self.workspaces
            .iter()
            .position(|workspace| workspace.groups.contains(&group_id))
            .expect("Unable to fetch workspace for requested group")
    }

    pub fn groups_for_workspace(&self, workspace_id: usize) -> Vec<usize> {
        self.workspaces[workspace_id].groups.clone()
    }

    pub fn toggle_active(&mut self, workspace_id: usize) {
        let workspace = self.get_workspace(workspace_id);
        workspace.active = !workspace.active;
    }

    fn get_workspace(&mut self, workspace_id: usize) -> &mut Workspace {
        self.workspaces
            .get_mut(workspace_id)
            .expect("Unable to fetch workspace for requested index")
    }

    fn get_group_index_in_workspace(&self, workspace_index: usize, group: usize) -> usize {
        self.workspaces[workspace_index]
            .groups
            .iter()
            .position(|g| g == &group)
            .expect("Unable to fetch groups index within workspace")
    }
}
