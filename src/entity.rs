use macroquad::prelude::*;
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
}
#[derive(Debug)]
pub struct EntSchema {
    name: String,
    sprite: String,
    albedo: Texture2D,
    normals: Texture2D,
    anims: HashMap<String, (u16, u16)>,
    sprite_size: (u16, u16),
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
    x: f32,
    y: f32,
    anim_index: u16,
    face_right: bool,
}

impl<'b> Ent<'b> {
    /*pub fn new(schema: String, x: f32, y: f32) -> Ent {
    }*/
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_name(&self) -> String {
        self.schema.name.to_owned()
    }
    pub fn get_schema(&self) -> &EntSchema {
        self.schema
    }
    fn get_anim(&mut self, animation: String) -> (u16, u16) {
        self.schema.get_anim(animation)
    }
    pub fn anim(&mut self, animation: String) {
        let inds = self.get_anim(animation);
        self.anim_index += 1;
        if self.anim_index > inds.1 {
            self.anim_index = inds.0;
        }
    }
    pub fn draw(&mut self, delta: f32, tick: bool, normal: bool) {
        //for i in 0..array.len() {
        //let dir = array[i].2;
        self.x += if self.face_right {
            2. * delta
        } else {
            -2. * delta
        };
        let x = self.x;
        let y = self.y;

        if x > 320. {
            //x-=2.;
            self.face_right = !self.face_right;
        } else if x < 0. {
            //x+=2.;
            //
            //i-=1;
            //array[i].0=-99.; //we'll mark it dead on the x position like as a weird work around
            //array_is_dirty=true;

            self.face_right = !self.face_right;
        }
        if !normal && tick {
            self.anim(String::from("Idle"));
        }
        // let max = (birb.width() / birb.height()) as u8;
        // println!("anim {}", delta);
        draw_texture_ex(
            if normal {
                self.schema.normals
            } else {
                self.schema.albedo
            }, //if dir {birb_n} else {birb_nf},
            self.x - 16.,
            self.y - 16., //+ 384.,
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

pub struct EntFactory {
    ent_map: HashMap<String, EntSchema>,
    default_ent_schema: EntSchema,
}

impl EntFactory {
    pub async fn new() -> EntFactory {
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
                sprite_size: (32, 32),
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
        };
        EntFactory {
            ent_map,
            default_ent_schema,
        }
    }
    pub fn create_ent(&self, schema: &str) -> Ent {
        //.or_insert(EntSchema::default());

        let sc = match self.ent_map.get(schema) {
            Some(o) => o,
            None => &self.default_ent_schema,
        };
        //(self.default_ent_schema);
        Ent {
            schema: sc,
            x: 0.,
            y: 0.,
            anim_index: 0,
            face_right: false,
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
