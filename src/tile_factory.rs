use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{read_dir, File},
    path::{Path, PathBuf},
};

use crate::{
    ent,
    tile::{Tile, TileBlock, TileTemplate},
};

pub async fn init_tiles() {
    let input_path = Path::new(".").join("tiles");
    //let input_path = format!("{}/entities/", env!("CARGO_MANIFEST_DIR"));
    let mut tile_map: HashMap<String, TileTemplate> = HashMap::new();
    println!("dir is {}", input_path.display());
    let dir: Vec<PathBuf> = read_dir(&input_path)
        .expect("Tile directory failed to load")
        .filter(Result::is_ok)
        .map(|e| e.unwrap().path())
        .collect();

    for entry in dir {
        println!("tile to load {}", entry.display());
        let f = File::open(&entry).expect("Failed opening a tile file");
        let ext = entry.extension().and_then(OsStr::to_str);

        let name = entry.file_stem().unwrap().to_str().unwrap().to_string();
        let path = entry.to_str().unwrap().to_string();
        match ext {
            Some(str) => {
                if str == "png" {
                    println!(
                        "tile extension is image {:?} and {:?}",
                        entry,
                        name.to_owned()
                    );
                    match tile_map.get_mut(&name) {
                        Some(prev) => {
                            prev.addTexture(&path, &name);
                        }
                        None => {
                            let tile = TileTemplate::simple(&path, &name).await;
                            tile_map.insert(name, tile);
                        }
                    };
                } else if str == "tile" {
                    println!("tile extension is ron tile {:?} and {:?}", entry, name);

                    match tile_map.get_mut(&name) {
                        Some(prev) => prev.addMeta(&path, &name),
                        None => {
                            let tile = TileTemplate::fromMeta(&path, &name);
                            tile_map.insert(name, tile);
                        }
                    }
                }

                //tile_map.insert(name, tile);
            }
            None => {}
        }
        // let schema: PreEntSchema = match from_reader(f) {
        //     Ok(x) => x,
        //     Err(e) => {
        //         println!("Failed to apply entity RON schema, defaulting: {}", e);
        //         //std::process::exit(1);
        //         PreEntSchema::default()
        //     }
        // };
    }
}
