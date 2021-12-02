use macroquad::prelude::Mat4;

pub struct Global {
    pub DEVELOPER_MODE: bool,
    pub MOUSE: (f32, f32),
    pub MATRIX: Mat4,
}

impl Default for Global {
    fn default() -> Global {
        Global {
            DEVELOPER_MODE: false,
            MOUSE: (0., 0.),
            MATRIX: Mat4::IDENTITY,
        }
    }
}
