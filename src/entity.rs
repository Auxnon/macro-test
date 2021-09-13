use ron::de::from_reader;
use serde::Deserialize;
use std::marker::PhantomData;
use std::{
    collections::{hash_map::Entry, HashMap},
    fs::{read_dir, File},
    path::{Path, PathBuf},
};

#[derive(Default, Debug, Deserialize)]
pub struct EntSchema {
    name: String,
    sprite: String,
    #[serde(default)]
    anims: HashMap<String, u16>,
}
impl EntSchema {
    pub fn get_anim(&self, name: String) -> u16 {
        match self.anims.get(&name) {
            Some(&o) => o,
            None => 0,
        }
    }
}

pub struct Ent<'b> {
    schema: &'b EntSchema,
    x: f32,
    y: f32,
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
}

pub struct EntFactory {
    ent_map: HashMap<String, EntSchema>,
}

impl EntFactory {
    pub fn new() -> EntFactory {
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
            let ent: EntSchema = match from_reader(f) {
                Ok(x) => x,
                Err(e) => {
                    println!("Failed to apply entity RON schema, defaulting: {}", e);
                    //std::process::exit(1);
                    EntSchema::default()
                }
            };
            println!("loaded entity as {}", ent.name);
            ent_map.insert(ent.name.to_owned(), ent);
        }
        EntFactory { ent_map }
    }
    pub fn create_ent(&mut self, schema: String) -> Ent {
        //.or_insert(EntSchema::default());
        let sc = self.ent_map.entry(schema).or_default();
        Ent {
            schema: sc,
            x: 0.,
            y: 0.,
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
