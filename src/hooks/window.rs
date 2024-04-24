use windows::Win32::{
    Foundation::HWND,
    UI::{
        Accessibility::HWINEVENTHOOK,
        WindowsAndMessaging::{CHILDID_SELF, EVENT_SYSTEM_FOREGROUND, OBJID_WINDOW},
    },
};

use crate::{
    data::{window::Window, workspace::Workspace},
    state::WORKSPACES,
    win_api::hook::set_event_hook,
};

pub fn init_hook() -> HWINEVENTHOOK {
    return set_event_hook(callback);
}

pub unsafe extern "system" fn callback(
    _: HWINEVENTHOOK,
    event: u32,
    hwnd: HWND,
    object_id: i32,
    child_id: i32,
    _: u32,
    _: u32,
) {
    match event {
        EVENT_SYSTEM_FOREGROUND => {
            if hwnd.0 == 0 {
                return;
            }

            if object_id != OBJID_WINDOW.0 {
                return;
            }

            if child_id != CHILDID_SELF as i32 {
                return;
            }

            let window_result = Window::from(hwnd);
            if window_result.is_none() {
                return;
            }

            let window = window_result.unwrap();
            // TODO: Add a border to the newly focused window here
            // let border_hwnd = create_window(
            //   WS_EX_TOOLWINDOW | WS_EX_LAYERED | WS_EX_TOPMOST | WS_EX_NOACTIVATE,
            //   "border",
            //   "border",
            //   WS_POPUP | WS_SYSMENU,
            //   window.rect.left,
            //   window.rect.top,
            //   window.rect.right - window.rect.left,
            //   window.rect.bottom - window.rect.top,
            //   get_main_module()
            // );

            // set_transparent(border_hwnd);

            let mut workspaces = WORKSPACES.lock().unwrap();
            let mut current_workspace = Workspace::current(&workspaces);
            let old_workspace_result = Workspace::find_by_window(&window, &mut workspaces);
            if old_workspace_result.is_some() {
                let mut old_workspace = old_workspace_result.unwrap();
                if old_workspace.id == current_workspace.id {
                    return;
                }
                old_workspace.remove_window(&window);
                workspaces[(&old_workspace.id - 1) as usize] = old_workspace.clone();
            }

            current_workspace.add_window(&window);
            workspaces[(current_workspace.id - 1) as usize] = current_workspace.clone();
        }
        _ => (),
    }
}
