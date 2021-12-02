use crate::ent_factory::EntSchema;
use crate::LuaEnt;
use macroquad::prelude::*;
use mlua::Function;
pub struct Ent<'b> {
    schema: &'b EntSchema,
    pub pos: Vec3,
    pub vel: Vec3,
    pub rot: Vec3,
    anim_index: u16,
    face_right: bool,
    logic: String, //can be empty, intended to override the entity schema for more variety, defaults to schema
    //logic_obj: dyn Logic,
    pub grounded: bool,
    pub edge_left: bool,
    pub edge_right: bool,
    pub primed: bool,
    pub logic_fn: mlua::Function<'b>, //fn(&mut Self, f32),
    flat: bool,                       //2D or 3D
    pub matrix: Mat4,
    pub accessory: Option<&'b EntSchema>,
    //pub evaluate: bool, //whether to apraise a dynamic change, currently just logic code, could be expensive
}

impl<'b> Ent<'b> {
    pub fn new(schema: &'b EntSchema, fuc: Function<'b>) -> Ent<'b> {
        let r = rand::gen_range(0., 1.);
        let mat = glam::Mat4::from_axis_angle(Vec3::new(0., 1., 0.), r * std::f32::consts::PI * 2.);
        Ent {
            schema,
            pos: Vec3::new(0., 0., 0.),
            vel: Vec3::new(0., 0., 0.),
            rot: Vec3::new(0., 0., 0.),
            anim_index: 0,
            face_right: false,
            //evaluate: false,
            grounded: false,
            primed: false,
            edge_left: false,
            edge_right: false,
            logic: String::new(),
            logic_fn: fuc,
            flat: true,
            matrix: mat,
            accessory: None,
        }
    }
    pub fn default(schema: &'b EntSchema, fuc: Function<'b>) -> Ent<'b> {
        Ent {
            schema,
            pos: Vec3::new(0., 0., 0.),
            vel: Vec3::new(0., 0., 0.),
            rot: Vec3::new(0., 0., 0.),
            anim_index: 0,
            face_right: false,
            //evaluate: false,
            grounded: false,
            primed: false,
            edge_left: false,
            edge_right: false,
            logic: String::new(),
            logic_fn: fuc,
            flat: true,
            matrix: glam::Mat4::IDENTITY,
            accessory: None,
        }
    }

    pub fn set_x(&mut self, x: f32) {
        self.pos.x = x;
    }

    pub fn set_xy(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
    }

    /* pub fn get_logic(&self) -> String {
        if self.logic.len() == 0 {
            self.schema.logic
        } else {
            self.logic
        }
    }*/
    pub fn set_logic() {}

    pub fn to_lua(&self) -> LuaEnt {
        LuaEnt {
            x: self.pos.x,
            y: self.pos.y,
            z: self.pos.z,
            vel_x: self.vel.x,
            vel_y: self.vel.y,
            vel_z: self.vel.z,

            rot_x: self.rot.x,
            rot_y: self.rot.y,
            rot_z: self.rot.z,
        }
    }

    pub fn run(&mut self, delta: f32) {
        //(self.logic_fn)(self, delta);
        let testo = self.to_lua();
        let res = self.logic_fn.call::<LuaEnt, LuaEnt>(testo);
        if res.is_err() {
            println!("bad return! 📜{} {:#?}", self.get_schema().logic, res.err());
            return;
        }

        let ent = res.unwrap();
        // println!("got back {} and {}", ent.x, ent.y);
        self.pos.x = ent.x;
        self.pos.y = ent.y;
        self.pos.z = ent.z;

        self.vel.x = ent.vel_x;
        self.vel.y = ent.vel_y;
        self.vel.z = ent.vel_z;

        self.rot.x = ent.rot_x;
        self.rot.y = ent.rot_y;
        self.rot.z = ent.rot_z;
        let quat = Quat::from_rotation_ypr(self.rot.y, self.rot.x, self.rot.z);
        self.matrix = Mat4::from_rotation_translation(quat, self.pos);

        //self.matrix = Mat4::from_translation(self.pos);
        //println!("matrix {}", self.matrix);
    }

    pub fn get_x(&self) -> f32 {
        self.pos.x
    }

    pub fn get_name(&self) -> String {
        self.schema.name.to_owned()
    }

    pub fn get_schema(&self) -> &EntSchema {
        self.schema
    }

    pub fn get_width(&self) -> f32 {
        self.schema.resource_size[0] as f32
    }

    pub fn get_height(&self) -> f32 {
        self.schema.resource_size[1] as f32
    }

    fn get_anim(&mut self, animation: String) -> (u16, u16) {
        self.schema.get_anim(animation)
    }

    pub fn is_flat(&self) -> bool {
        self.schema.flat
    }

    pub fn anim(&mut self, animation: String) {
        let inds = self.schema.get_anim(animation);
        self.anim_index += 1;
        if self.anim_index > inds.1 {
            self.anim_index = inds.0;
        }
    }

    pub fn draw(&mut self, delta: f32, tick: bool, normal: bool, dimensional: bool) {
        let x = self.pos.x;
        let y = self.pos.y;

        //if normal && tick {}
        if tick {
            self.anim("Idle".to_owned());
        }

        if dimensional {
            if self.schema.flat {
                draw_plane(
                    vec3(0., -0.25, 0.), //1. + 1. / 4., 0., -1. / 4.),
                    vec2(1. / 4., 1. / 4.),
                    self.schema.albedo,
                    WHITE,
                );
                match self.accessory {
                    Some(ac) => {
                        let ar = std::f32::consts::PI * 2. * (self.anim_index as f32 / 5.);
                        //println!("index {}", self.anim_index);
                        draw_plane(
                            vec3(-0.4 + ar.cos() * 0.2, -0.26, ar.sin() * 0.2), //1. + 1. / 4., 0., -1. / 4.),
                            vec2(1. / 4., 1. / 4.),
                            ac.albedo,
                            WHITE,
                        );
                    }
                    None => {}
                }
            } else {
                for m in &self.schema.mesh {
                    draw_mesh(m);
                }
            }
        } else {
            draw_texture_ex(
                if normal {
                    self.schema.normals
                } else {
                    self.schema.albedo
                }, //if dir {birb_n} else {birb_nf},
                (self.pos.x as f32).floor(), // - self.schema.sprite_size.0
                (self.pos.y).floor(),        //+ 384., //- self.schema.sprite_size.1 as f32
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(
                        (self.anim_index * self.schema.resource_size[0]) as f32,
                        0.,
                        self.schema.resource_size[0].into(),
                        self.schema.resource_size[1].into(),
                    )),
                    flip_x: self.face_right,
                    ..Default::default()
                },
            );
        }
    }
}
