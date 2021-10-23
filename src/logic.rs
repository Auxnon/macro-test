use crate::controls;
use crate::lua_define;
use crate::lua_define::LuaCore;
use crate::Ent;

pub fn physics(ent: &mut Ent, delta: f32) {
    ent.pos.y += ent.vel.y * delta;
    ent.pos.x += ent.vel.x * delta;
    if ent.pos.y < 192. && !ent.grounded {
        ent.vel.y += 800. * delta;
        // ent.grounded = true;
    } else {
        ent.grounded = true;
    }
    if ent.grounded {
        ent.vel.y = 0.;
        if ent.vel.x.abs() > 0.1 {
            ent.vel.x *= 0.75;
        } else {
            ent.vel.x = 0.
        }
    }
}

pub fn player_logic(ent: &mut Ent, delta: f32) {
    controls::player_controls(ent);
    physics(ent, delta);
}

pub fn get_logic(str: String, lua_core: LuaCore) -> u32 {
    //fn(&mut Ent, f32)
    //lua_core.load(str)
    0
    // match str.to_lowercase().as_ref() {
    //     "player" => player_logic,
    //     _ => lua_core.get(str)
    //     //|ent, delta| {
    //         //ent.pos.y += 0.4 * delta;
    //         // lua_define.get(str);
    //         //lua_define.load(str);
    //     //},
    // }
}
/*
pub struct LogicManager {}
impl LogicManager {
    pub fn get(str: String) -> dyn Logic {
        Player {}
    }
}*/
