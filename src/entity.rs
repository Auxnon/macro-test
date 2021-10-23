use crate::logic::get_logic;
use macroquad::prelude::*;

use crate::lua_define::LuaCore;
use mlua::{UserData, UserDataMethods};
use ron::de::from_reader;
use serde::Deserialize;
use std::marker::PhantomData;
use std::{
    collections::{hash_map::Entry, HashMap},
    fs::{read_dir, File},
    path::{Path, PathBuf},
};

#[derive(Default, Debug, Deserialize)]
pub struct PreEntSchema {
    name: String,
    sprite: String,
    #[serde(default)]
    anims: HashMap<String, (u16, u16)>,
    sprite_size: (u16, u16),
    logic: String,
}
#[derive(Debug)]
pub struct EntSchema {
    name: String,
    sprite: String,
    albedo: Texture2D,
    normals: Texture2D,
    anims: HashMap<String, (u16, u16)>,
    sprite_size: (u16, u16),
    logic: String,
}
impl EntSchema {
    pub fn get_anim(&self, name: String) -> (u16, u16) {
        match self.anims.get(&name) {
            Some(&o) => o,
            None => (0, 0),
        }
    }
}

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
struct LuaEnt<'b> {
    ent: Ent<'b>,
}
// impl<T: IAnimalData> Animal<T> {
// impl<'b> UserData for LuaEnt<'b> {
//     fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
//         methods.add_method("add_x", |_, this, ()| Ok(Self.ent.set_x(10.)));

//         methods.add_async_function(
//             "read",
//             |lua, (this, size): (AnyUserData, usize)| async move {
//                 let mut this = this.borrow_mut::<Self>()?;
//                 let mut buf = vec![0; size];
//                 let n = this.0.read(&mut buf).await?;
//                 buf.truncate(n);
//                 lua.create_string(&buf)
//             },
//         );

//         methods.add_async_function(
//             "write",
//             |_, (this, data): (AnyUserData, LuaString)| async move {
//                 let mut this = this.borrow_mut::<Self>()?;
//                 let n = this.0.write(&data.as_bytes()).await?;
//                 Ok(n)
//             },
//         );

//         methods.add_async_function("close", |_, this: AnyUserData| async move {
//             let mut this = this.borrow_mut::<Self>()?;
//             this.0.shutdown().await?;
//             Ok(())
//         });
//     }
// }
impl<'b> Ent<'b> {
    /*pub fn new(schema: String, x: f32, y: f32) -> Ent {
    }*/
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
        let res = self.logic_fn.call::<_, f32>(self.pos.y);
        if res.is_err() {
            println!("bad return!");
            return;
        }
        let y = res.unwrap();
        println!("got back {}", y);
        self.pos.y = y;
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

pub struct EntFactory<'a> {
    ent_map: HashMap<String, EntSchema>,
    default_ent_schema: EntSchema,
    lua_core: LuaCore<'a>,
}

impl<'a> EntFactory<'a> {
    pub async fn new() -> EntFactory<'a> {
        let input_path = Path::new(".").join("entities");
        //let input_path = format!("{}/entities/", env!("CARGO_MANIFEST_DIR"));
        let mut ent_map = HashMap::new();
        println!("dir is {}", input_path.display());
        let dir: Vec<PathBuf> = read_dir(&input_path)
            .expect("Entity directory failed to load")
            .filter(Result::is_ok)
            .map(|e| e.unwrap().path())
            .collect();

        for entry in dir {
            println!("entity to load {}", entry.display());
            let f = File::open(&entry).expect("Failed opening an entity file");
            let schema: PreEntSchema = match from_reader(f) {
                Ok(x) => x,
                Err(e) => {
                    println!("Failed to apply entity RON schema, defaulting: {}", e);
                    //std::process::exit(1);
                    PreEntSchema::default()
                }
            };
            let text = format!("assets/{}.png", schema.sprite);
            let ntext = format!("assets/{}_n.png", schema.sprite);
            //println!("loaded texture {}", text);
            let albedo = load_texture(&text[..]).await.unwrap_or(Texture2D::empty());
            //println!(" texture width {}", albedo.width());
            let normals = load_texture(&ntext[..]).await.unwrap_or(Texture2D::empty());

            normals.set_filter(FilterMode::Nearest);
            albedo.set_filter(FilterMode::Nearest);
            let ent = EntSchema {
                name: schema.name,
                anims: schema.anims,
                sprite: schema.sprite,
                albedo,
                normals,
                logic: schema.logic,
                sprite_size: schema.sprite_size,
            };
            println!("loaded entity as {}", ent.name);
            ent_map.insert(ent.name.to_owned(), ent);
        }
        let default_ent_schema = EntSchema {
            name: String::from("NA"),
            anims: HashMap::new(),
            sprite: String::from("none"),
            albedo: Texture2D::empty(),
            normals: Texture2D::empty(),
            sprite_size: (32, 32),
            logic: "".to_string(),
        };
        EntFactory {
            ent_map,
            default_ent_schema,
            lua_core: LuaCore::new(),
        }
    }
    pub fn create_ent(&self, schema: &str) -> Ent {
        //.or_insert(EntSchema::default());

        let sc = match self.ent_map.get(schema) {
            Some(o) => o,
            None => &self.default_ent_schema,
        };
        //(self.default_ent_schema);
        //fn basic(ent: &mut Ent) {}
        //let f = get_logic("player".to_owned());
        //let r = rand::gen_range(0, 2);
        //let fuc = get_logic(sc.logic.clone(), self.lua_core);
        let fuc = self.lua_core.load(sc.logic.clone());
        Ent {
            schema: sc,
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
        }
    }
}

/*
impl Ent {
    pub fn new(x: f32, y: f32) -> Ent {
        Ent { x, y }
    }
}*/

/*
#[derive(Debug, Deserialize)]
struct Config {
    boolean: bool,
    float: f32,
    map: HashMap<u8, char>,
    nested: Nested,
    tuple: (u32, u32),
    vec: Vec<Nested>,
}*/
