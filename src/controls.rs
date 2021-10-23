use crate::Ent;
use crate::Global;
use macroquad::prelude::*;

#[path = "menu.rs"]
mod menu;

static SPEED: f32 = 40.;
static JUMP_SPEED: f32 = 300.;

pub fn cycle(globals: &mut Global) {
    if is_key_pressed(KeyCode::GraveAccent) {
        globals.DEVELOPER_MODE = !globals.DEVELOPER_MODE;
    }
    menu::cycle(globals);
}
pub fn player_controls(player: &mut Ent) {
    if player.grounded {
        let modifier: f32 = if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
            2.
        } else {
            1.
        };
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {}

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            player.vel.x -= SPEED * modifier;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            player.vel.x += SPEED * modifier;
        }
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) || is_key_down(KeyCode::Space) {
        if player.primed {
            if player.grounded {
                player.vel.y = -JUMP_SPEED;
                //player.pos.y -= 10.;
                player.grounded = false;
                player.primed = false;
            } else if player.edge_left {
                player.vel.y = -JUMP_SPEED * 0.707;
                player.vel.x = -JUMP_SPEED * 0.707;
                player.edge_left = false;
                player.primed = false;
                player.grounded = false;
            } else if player.edge_right {
                player.vel.y = -JUMP_SPEED * 0.707;
                player.vel.x = JUMP_SPEED * 0.707;
                player.edge_right = false;
                player.primed = false;
                player.grounded = false;
            }
        }
    } else {
        player.primed = true;
    }
    if is_key_down(KeyCode::Key1) {
        player.vel.y = 0.;
        player.vel.x = 0.;
        player.pos.y = 60.;
        player.pos.x = 160.;
    }
    if is_key_down(KeyCode::Key2) {
        player.vel.y = -200.;
        player.grounded = false;
    }
}
