use macroquad::prelude::*;
use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

use crate::Layer;

pub async fn create_template(texture: &str) -> TileTemplate {
    let out = [texture, ".png"].join("");
    let nout = [texture, "_n.png"].join("");
    let textur: &str = &*out;
    let normal: &str = &*nout; //explicit reborrowing, just to covnert our dynamic String to a static str, wow!
                               //s.push_str("_n");

    let t = load_texture(textur).await.unwrap();
    let n = load_texture(normal).await.unwrap();

    TileTemplate {
        name: "old".to_string(),
        dimensional: false,
        flow: false,
        size: 16,
        tile_names: vec![],
        countx: (t.width() / 16.) as u32,
        county: (t.height() / 16.) as u32,
        texture: t,
        normals: n,
    }
}

#[derive(Default, Debug, Deserialize)]
pub struct PreTileTemplate {
    name: String,
    #[serde(default)]
    tile_names: Vec<String>,
    #[serde(default)]
    dimensional: bool,
    #[serde(default)]
    flow: bool,
    #[serde(default = "default_tile_size")]
    size: u16,
}

fn default_tile_size() -> u16 {
    16 as u16
}

pub struct TileTemplate {
    name: String,
    texture: Texture2D,
    normals: Texture2D,
    countx: u32,
    county: u32,
    tile_names: Vec<String>,
    dimensional: bool,
    flow: bool,
    size: u16,
}

impl TileTemplate {
    pub async fn simple(path: &String, texture: &String) -> TileTemplate {
        if (texture.ends_with("_n")) {
            let n = load_texture(path).await.unwrap();
            TileTemplate {
                name: texture.to_string(),
                countx: (n.width() / 16.) as u32,
                county: (n.height() / 16.) as u32,
                texture: Texture2D::empty(),
                normals: n,
                tile_names: vec![],
                dimensional: false,
                flow: false,
                size: 16,
            }
        } else {
            let t = load_texture(path).await.unwrap();
            TileTemplate {
                name: texture.to_string(),
                countx: (t.width() / 16.) as u32,
                county: (t.height() / 16.) as u32,
                texture: t,
                normals: Texture2D::empty(),
                tile_names: vec![],
                dimensional: false,
                flow: false,
                size: 16,
            }
        }
    }

    pub async fn addTexture(&mut self, path: &String, texture: &String) {
        if (texture.ends_with("_n")) {
            let n = load_texture(path).await.unwrap();
            self.normals = n;
            if self.countx == 0 {
                self.countx = (n.width() / 16.) as u32;
                self.county = (n.height() / 16.) as u32;
            }
        } else {
            let t = load_texture(path).await.unwrap();
            self.texture = t;
            if self.countx == 0 {
                self.countx = (t.width() / 16.) as u32;
                self.county = (t.height() / 16.) as u32;
            }
        }
    }

    pub fn fromMeta(path: &String, texture: &String) -> TileTemplate {
        match TileTemplate::loadMeta(path) {
            Some(m) => TileTemplate {
                name: m.name.to_string(),
                countx: 0,
                county: 0,
                texture: Texture2D::empty(),
                normals: Texture2D::empty(),
                tile_names: m.tile_names,
                dimensional: m.dimensional,
                flow: m.flow,
                size: m.size,
            },
            None => TileTemplate {
                name: texture.to_string(),
                countx: 0,
                county: 0,
                texture: Texture2D::empty(),
                normals: Texture2D::empty(),
                tile_names: vec![],
                dimensional: false,
                flow: false,
                size: 16,
            },
        }
    }
    pub fn addMeta(&mut self, path: &String, texture: &String) {
        let meta = TileTemplate::loadMeta(path);
        if (meta.is_some()) {
            let m = meta.unwrap();
            self.dimensional = m.dimensional;
            self.flow = m.flow;
            self.name = m.name;
            self.tile_names = m.tile_names;
        }
    }
    fn loadMeta(path: &String) -> Option<PreTileTemplate> {
        let f = File::open(path).expect("Failed opening a tile file");
        match from_reader(f) {
            Ok(x) => Some(x),
            Err(e) => {
                println!("Failed to apply tile RON schema, defaulting: {}", e);
                //std::process::exit(1);
                None
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub id: u8,
    pub x: u16,
    pub y: u16,
    flag: u16,
}
impl Tile {
    //   fn new()->Tile{
    //       Tile{}
    //   }
    pub fn set_flag(&mut self, flag: bool) {
        self.flag = 2;
    }
    pub fn get_flag(&mut self) -> bool {
        self.flag > 0
    }
}
#[derive(Copy, Clone)]
pub struct TileBlock<'tb> {
    x: u32,
    y: u32,
    array: [[Tile; 20]; 12],
    template: &'tb TileTemplate,
}
fn int_to_tile(tiles: [[u8; 20]; 12]) -> [[Tile; 20]; 12] {
    let mut ar = [[Tile {
        id: 10,
        x: 0,
        y: 0,
        flag: 0,
    }; 20]; 12];
    for i in 0..20 {
        for j in 0..12 {
            ar[j][i].x = i as u16;
            ar[j][i].y = j as u16;
            ar[j][i].id = tiles[j][i];
        }
    }
    println!("inside {}", ar[0][0].id);
    ar
}

impl<'tb> TileBlock<'tb> {
    pub fn new(x: u32, y: u32, template: &TileTemplate, tiles: [[u8; 20]; 12]) -> TileBlock {
        //let ar = [[Tile { id: 10, x: 0, y: 0 }; 16]; 16];
        TileBlock {
            x,
            y,
            array: int_to_tile(tiles),
            template,
        }
    }
    pub fn get_at_pos(&mut self, x: usize, y: usize) -> Option<&mut Tile> {
        if x >= 20 || y >= 12 {
            None
        } else {
            Some(&mut self.array[y as usize][x as usize])
        }
    }
    pub fn get_in_range(&mut self, x: f32, y: f32, w: f32, h: f32) -> Vec<&mut Tile> {
        let sx = (x / 16.).floor() as usize;
        let sy = (y / 16.).floor() as usize;
        let ex = (((x + w) / 16.).ceil() as usize) - sx;
        let ey = (((y + h) / 16.).ceil() as usize) - sy;
        let mut v = vec![];
        //println!("x {} y{} sx{} sy{}", x, y, sx, sy);
        for i in self.array.iter_mut().skip(sy).take(ey) {
            for j in i.iter_mut().skip(sx).take(ex) {
                // *x += 1;
                v.push(j);
            }
        }
        // for i in sx..ex {
        //     for j in sy..ey {
        //         if i < 20 || j < 12 {
        //             v.push(&mut two[i as usize][j as usize])
        //         }
        //     }
        // }
        v
    }
    pub fn set(&mut self, id: u8, x: u16, y: u16) {
        // let t = Tile { id, x, y };
        self.array[y as usize][x as usize].id = id;
    }
    pub fn pos(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
    pub fn pos_add(&mut self, x: u32, y: u32) {
        self.x += x;
        self.y += y;
    }
    pub fn draw_normals(&mut self, lx: f32, ly: f32) {
        self._draw(lx, ly, self.template.normals);
    }
    pub fn draw(&mut self, lx: f32, ly: f32) {
        self._draw(lx, ly, self.template.texture);
    }

    fn _draw(&mut self, lx: f32, ly: f32, texture: Texture2D) {
        let ox = lx + self.x as f32;
        let oy = ly + self.y as f32;
        for i in 0..self.array[0].len() {
            for j in 0..self.array.len() {
                //self.array[i][j]
                //draw_texture_ex()
                let id = self.array[j][i].id;
                let x = (id as u32 % self.template.countx) as f32;
                let y = (id as u32 / self.template.countx) as f32;
                // if i == 0 {
                //     println!("xy {} {}", (i * 16), (j * 16));
                // }
                draw_texture_ex(
                    texture,
                    (i * 16) as f32 + ox,
                    (j * 16) as f32 + oy, //384. +
                    if self.array[j][i].flag > 0 {
                        RED
                    } else {
                        WHITE
                    },
                    DrawTextureParams {
                        source: Some(Rect::new(x * 16., y * 16., 16., 16.)),
                        //flip_x: dir,
                        ..Default::default()
                    },
                );
                if self.array[j][i].flag > 0 {
                    self.array[j][i].flag -= 1;
                }
            }
        }
    }
    pub fn draw3(&mut self, lx: f32, ly: f32, lz: f32) {
        for i in 0..self.array[0].len() {
            for j in 0..self.array.len() {
                //self.array[i][j]
                //draw_texture_ex()
                let id = self.array[j][i].id;
                let x = (id as u32 % self.template.countx) as f32;
                let y = (id as u32 / self.template.countx) as f32;
                // if i == 0 {
                //     println!("xy {} {}", (i * 16), (j * 16));
                // }
                draw_plane(
                    Vec3::new(self.x as f32 * 1., 0., self.y as f32 * 1.),
                    Vec2::new(1., 1.),
                    self.template.texture,
                    if self.array[j][i].flag > 0 {
                        RED
                    } else {
                        WHITE
                    },
                );
                // draw_texture_ex(
                //     ,
                //     (i * 16) as f32 + ox,
                //     (j * 16) as f32 + oy, //384. +
                //     if self.array[j][i].flag > 0 {
                //         RED
                //     } else {
                //         WHITE
                //     },
                //     DrawTextureParams {
                //         source: Some(Rect::new(x * 16., y * 16., 16., 16.)),
                //         //flip_x: dir,
                //         ..Default::default()
                //     },
                // );
                if self.array[j][i].flag > 0 {
                    self.array[j][i].flag -= 1;
                }
            }
        }
    }
}
