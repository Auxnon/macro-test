use ron::de::from_reader;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::{read_dir, File},
    path::{Path, PathBuf},
};

//mut ENT_MAP: HashMap<String, EntSchema>;

#[derive(Default, Debug, Deserialize)]
pub struct EntSchema {
    name: String,
    sprite: String,
    anims: Option<HashMap<String, u16>>,
}

pub struct Ent {
    schema: EntSchema,
    x: f32,
    y: f32,
}
impl Ent {
    pub fn new(schema: String, x: f32, y: f32) -> Ent {
        let schema = ENT_MAP.entry(schema).or_insert(EntSchema::default());
        Ent {
            schema: EntSchema::default(),
            x,
            y,
        }
    }
}
pub struct EntFactory {
}
imp EntFactory {
    pub fn load_ents(&self) -> HashMap<String, EntSchema> {
        let input_path = Path::new(".").join("entities");
        //let input_path = format!("{}/entities/", env!("CARGO_MANIFEST_DIR"));
        let ENT_MAP: HashMap<String, EntSchema> = HashMap::new();
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
            ENT_MAP.insert(ent.name, ent);
            println!("loaded entity as {}", ent.name);
        }
        self.ENT_MAP
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
