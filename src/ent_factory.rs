use crate::Ent;
use crate::{lua_define::LuaCore, three_loader};
use gltf::Texture;
use macroquad::prelude::*;
use once_cell::sync::OnceCell;
use ron::de::from_reader;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::{
    collections::HashMap,
    fs::{read_dir, File},
    path::{Path, PathBuf},
};

#[derive(Default, Debug, Deserialize)]
pub struct PreEntSchema {
    name: String,
    resource: String,
    #[serde(default)]
    anims: HashMap<String, (u16, u16)>,
    #[serde(default)]
    resource_size: Vec<u16>,
    logic: String,
}

pub struct EntSchema {
    pub name: String,
    pub resource: String,
    pub albedo: Texture2D,
    pub normals: Texture2D,
    pub mesh: Vec<Mesh>,
    pub anims: HashMap<String, (u16, u16)>,
    pub resource_size: Vec<u16>,
    pub logic: String,
    pub flat: bool,
}
impl EntSchema {
    pub fn get_anim(&self, name: String) -> (u16, u16) {
        match self.anims.get(&name) {
            Some(&o) => o,
            None => (0, 0),
        }
    }
}

pub struct EntFactory {
    ent_map: HashMap<String, EntSchema>,
    default_ent_schema: EntSchema,
}

impl<'a> EntFactory {
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
            let mut ent;

            if (schema.resource_size.len() > 2) {
                //then it's a 3d resource!
                let text = format!("assets/{}.glb", schema.resource);
                let mesh = three_loader::load(&text);

                ent = EntSchema {
                    name: schema.name,
                    anims: schema.anims,
                    resource: schema.resource,
                    albedo: Texture2D::empty(),
                    normals: Texture2D::empty(),
                    mesh,
                    logic: schema.logic,
                    resource_size: schema.resource_size,
                    flat: false,
                };
            } else {
                let text = format!("assets/{}.png", schema.resource);
                let ntext = format!("assets/{}_n.png", schema.resource);
                //println!("loaded texture {}", text);
                let albedo = load_texture(&text[..]).await.unwrap_or(Texture2D::empty());
                //println!(" texture width {}", albedo.width());
                let normals = load_texture(&ntext[..]).await.unwrap_or(Texture2D::empty());
                let mesh = vec![Mesh {
                    vertices: [].to_vec(),
                    indices: [].to_vec(),
                    texture: Some(Texture2D::empty()),
                }];
                normals.set_filter(FilterMode::Nearest);
                albedo.set_filter(FilterMode::Nearest);
                ent = EntSchema {
                    name: schema.name,
                    anims: schema.anims,
                    resource: schema.resource,
                    albedo,
                    normals,
                    mesh,
                    logic: schema.logic,
                    resource_size: schema.resource_size,
                    flat: true,
                };
            }

            println!("loaded entity as {}", ent.name);
            ent_map.insert(ent.name.to_owned(), ent);
        }
        let default_ent_schema = EntSchema {
            name: String::from("NA"),
            anims: HashMap::new(),
            resource: String::from("none"),
            albedo: Texture2D::empty(),
            normals: Texture2D::empty(),
            mesh: vec![Mesh {
                vertices: [].to_vec(),
                indices: [].to_vec(),
                texture: Some(Texture2D::empty()),
            }],
            resource_size: [32, 32, 0].to_vec(),
            logic: "".to_string(),
            flat: false,
        };
        EntFactory {
            ent_map,
            default_ent_schema,
            //lua_core: LuaCore::new(self),
        }
    }
    // pub fn create_empty_ent(&self) {
    //     Ent::default(self.default_ent_schema, mlua::Function::clone(&self))
    // }
    pub fn get_schema(&self, schema: &str) -> &EntSchema {
        match self.ent_map.get(schema) {
            Some(o) => o,
            None => &self.default_ent_schema,
        }
    }
    pub fn create_ent(
        &'a self,
        schema: &String,
        lua_core: Arc<Mutex<OnceCell<LuaCore<'a>>>>,
    ) -> Ent<'a> {
        //.or_insert(EntSchema::default());

        let sc = self.get_schema(schema);
        //(self.default_ent_schema);
        //fn basic(ent: &mut Ent) {}
        //let f = get_logic("player".to_owned());
        //let r = rand::gen_range(0, 2);
        //let fuc = get_logic(sc.logic.clone(), self.lua_core);
        let fuc = lua_core
            .lock()
            .unwrap()
            .get()
            .unwrap()
            .get(sc.logic.clone());
        println!("::ent:: we loaded func for {}", sc.logic.clone());
        Ent::new(sc, fuc.clone())
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
    pub fn test(&self) -> u32 {
        0
    }
}
