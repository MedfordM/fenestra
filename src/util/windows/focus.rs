use crate::data::actions::Focus;
use crate::data::application::Application;
use crate::win_api;
use crate::win_api::window::set_foreground_window;

pub fn focus_direction(focus_action: &Focus) {
    let nearest_window_in_direction: Application =
        win_api::window::find_nearest_window_in_direction(&focus_action.direction);
    set_foreground_window(nearest_window_in_direction);
}
