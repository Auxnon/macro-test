
use crate::ent_factory::EntSchema;
use macroquad::prelude::*;
use mlua::Function;
use crate::LuaEnt;
pub struct Ent<'b> {
    schema: &'b EntSchema,
    pub pos: Vec2,
    pub vel: Vec2,
    anim_index: u16,
    face_right: bool,
    logic: String, //can be empty, intended to override the entity schema for more variety, defaults to schema
    //logic_obj: dyn Logic,
    pub grounded: bool,
    pub edge_left: bool,
    pub edge_right: bool,
    pub primed: bool,
    pub logic_fn: mlua::Function<'b>, //fn(&mut Self, f32),
    pub evaluate: bool, //whether to apraise a dynamic change, currently just logic code, could be expensive
}


impl<'b> Ent<'b> {
    pub fn new(schema: &'b EntSchema,fuc:Function<'b> ) -> Ent<'b> {
    Ent {
        schema,
        pos: Vec2::new(0., 0.),
        vel: Vec2::new(0., 0.),
        anim_index: 0,
        face_right: false,
        evaluate: false,
        grounded: false,
        primed: false,
        edge_left: false,
        edge_right: false,
        logic: String::new(),
        logic_fn: fuc,
    }}
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

    pub fn run(&mut self, delta: f32) {
        //(self.logic_fn)(self, delta);

        let testo = LuaEnt {
            x: self.pos.x,
            y: self.pos.y,
            vel_x: self.vel.x,
            vel_y:self.vel.y
        };


        let res = self.logic_fn.call::<LuaEnt, LuaEnt>((testo));
        if res.is_err() {
            println!("bad return! 📜{} {:#?}", self.get_schema().logic, res.err());
            return;
        }

        let ent = res.unwrap();
        println!("got back {} and {}", ent.x, ent.y);
        self.pos.x = ent.x;

        self.pos.y = ent.y;
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
        self.schema.sprite_size.0 as f32
    }
    pub fn get_height(&self) -> f32 {
        self.schema.sprite_size.1 as f32
    }
    fn get_anim(&mut self, animation: String) -> (u16, u16) {
        self.schema.get_anim(animation)
    }
    pub fn anim(&mut self, animation: String) {
        let inds = self.schema.get_anim(animation);
        self.anim_index += 1;
        if self.anim_index > inds.1 {
            self.anim_index = inds.0;
        }
    }
    pub fn draw(&mut self, delta: f32, tick: bool, normal: bool) {
        //for i in 0..array.len() {
        //let dir = array[i].2;
        // if self.schema.logic.chars().count() <= 0 {
        //     self.pos.x += if self.face_right {
        //         2. * delta
        //     } else {
        //         -2. * delta
        //     };
        // }
        let x = self.pos.x;
        let y = self.pos.y;

        if x > 320. {
            self.face_right = !self.face_right;
        } else if x < 0. {
            self.face_right = !self.face_right;
        }
        if normal && tick {
            self.anim("Idle".to_owned());
        }
        // let max = (birb.width() / birb.height()) as u8;
        // println!("anim {:?}", self.schema.sprite_size);
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
                    (self.anim_index * self.schema.sprite_size.0) as f32,
                    0.,
                    self.schema.sprite_size.0.into(),
                    self.schema.sprite_size.1.into(),
                )),
                flip_x: self.face_right,
                ..Default::default()
            },
        );
    }
}