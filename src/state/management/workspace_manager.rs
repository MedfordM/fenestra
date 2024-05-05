use crate::data::workspace::Workspace;

pub struct WorkspaceManager {
    workspaces: Vec<Workspace>
}

impl WorkspaceManager {
    pub fn new(workspaces: Vec<Workspace>) -> Self {
        Self { workspaces }
    }

    // TODO: This should take a vector of workspaces belonging to a monitor
    pub fn get_current_workspace(&self, workspace_ids: &Vec<usize>) -> usize {
        self.workspaces.iter().filter(|workspace| workspace_ids.contains(&workspace.index))
            .position(|workspace| workspace.focused)
            .expect("Unable to fetch current workspace for requested ids")
    }
    
    pub fn focus_workspace(&mut self, workspace_id: usize) -> &Vec<usize> {
        let workspace = self.get_workspace(workspace_id);
        workspace.focused = true;
        return self.groups_for_workspace(workspace_id);
    }
    
    // TODO: Unfocus workspace
    
    pub fn groups_for_workspace(&self, workspace_id: usize) -> &Vec<usize> {
        &self.workspaces[workspace_id].groups
    }
    
    fn get_workspace(&mut self, workspace_id: usize) -> &mut Workspace {
        self.workspaces
            .get_mut(workspace_id)
            .expect("Unable to fetch workspace for requested index")
    }
    
}