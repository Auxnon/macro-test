use crate::Ent;

pub trait Logic: Sized {
    fn run(&self, ent: &Ent) {}
}
/*
impl dyn Logic {
    pub fn run(&self, ent: &Ent) {}
}*/

pub struct Player {}
impl Logic for Player {
    fn run(&self, ent: &Ent) {}
}

pub fn get_logic(str: String) -> fn(&mut Ent) {
    match str.to_lowercase().as_ref() {
        "player" => |ent: &mut Ent| {
            ent.pos.x += 0.6;
            ent.pos.y -= 0.2;
            // controls::cycle(globals: &mut Global)
        },
        _ => |ent: &mut Ent| {
            ent.pos.y += 0.4;
        },
    }
}
/*
pub struct LogicManager {}
impl LogicManager {
    pub fn get(str: String) -> dyn Logic {
        Player {}
    }
}*/
