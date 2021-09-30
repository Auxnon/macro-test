use crate::Ent;
use crate::Global;
use macroquad::prelude::*;

#[path = "menu.rs"]
mod menu;

pub fn cycle(globals: &mut Global) {
    //player
    if is_key_pressed(KeyCode::GraveAccent) {
        globals.DEVELOPER_MODE = !globals.DEVELOPER_MODE;
    }
    if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
        // player.pos -= 1.;
    }
    if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
        println!("L");
    }
    if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
        println!("L");
    }
    if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
        println!("L");
    }
    menu::cycle(globals);
}
