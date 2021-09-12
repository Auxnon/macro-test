pub struct Anim {
    name: str,
    start: u16,
    end: u16,
}
pub struct Animations {
    anims: Vec<Anim>,
}

pub trait Animations {
    fn default() {}
}

impl Cloud {
    pub fn new(x: f32, y: f32) -> Animations {
        Ent { x, y }
    }
}
