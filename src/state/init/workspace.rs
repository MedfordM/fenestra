use crate::data::monitor::Monitor;
use crate::data::workspace::Workspace;
use std::cell::RefCell;
use std::sync::Arc;

pub fn init_workspaces(monitor_ref: &Arc<RefCell<Monitor>>) -> Vec<Arc<RefCell<Workspace>>> {
    let mut workspaces = vec![];
    for i in 1..10 {
        let workspace = Workspace::new(i, Arc::clone(monitor_ref));
        workspaces.push(Arc::new(RefCell::new(workspace)));
    }
    return workspaces;
}
