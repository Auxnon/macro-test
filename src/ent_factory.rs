
use macroquad::prelude::*;
use std::{
    collections::HashMap,
    fs::{read_dir, File},
    path::{Path, PathBuf}};
use serde::Deserialize;
use crate::lua_define::LuaCore;
use ron::de::from_reader;
use crate::Ent;

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
    pub name: String,
    pub sprite: String,
    pub albedo: Texture2D,
    pub normals: Texture2D,
    pub anims: HashMap<String, (u16, u16)>,
    pub sprite_size: (u16, u16),
    pub logic: String,
}
impl EntSchema {
    pub fn get_anim(&self, name: String) -> (u16, u16) {
        match self.anims.get(&name) {
            Some(&o) => o,
            None => (0, 0),
        }
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
        let fuc = self.lua_core.get(sc.logic.clone());
        println!("::ent:: we loaded func for {}", sc.logic.clone());
        Ent::new(sc,fuc)
        // Ent {
        //     schema: sc,
        //     pos: Vec2::new(0., 0.),
        //     vel: Vec2::new(0., 0.),
        //     anim_index: 0,
        //     face_right: false,
        //     evaluate: false,
        //     grounded: false,
        //     primed: false,
        //     edge_left: false,
        //     edge_right: false,
        //     logic: String::new(),
        //     logic_fn: fuc,
        // }
    }
}