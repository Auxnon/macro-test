use crate::Global;
use macroquad::prelude::*;

#[path = "menu.rs"]
mod menu;

pub fn cycle(globals: &mut Global) {
    if is_key_pressed(KeyCode::GraveAccent) {
        globals.DEVELOPER_MODE = !globals.DEVELOPER_MODE;
    }
    menu::cycle(globals);
}
