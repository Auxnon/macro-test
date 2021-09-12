use crate::Ent;
use crate::TileBlock;

pub struct Layer {
    x: f32,
    y: f32,
    scale: f32,
    tiles: Vec<TileBlock>,
    ents: Vec<Ent>,
}

impl Layer {
    /*pub fn new(scale: f32) -> Layer {
        //let ar = [[Tile { id: 10, x: 0, y: 0 }; 16]; 16];

        Layer::new(scale, 0, 0)
    }*/
    pub fn new(scale: f32, x: f32, y: f32) -> Layer {
        //let ar = [[Tile { id: 10, x: 0, y: 0 }; 16]; 16];
        Layer {
            x,
            y,
            scale,
            tiles: vec![],
            ents: vec![],
        }
    }
    pub fn add_tile(&mut self, mut tile: TileBlock) {
        self.tiles.push(tile);
    }
    pub fn remove_tile(&mut self, index: usize) {
        self.tiles.remove(index);
    }
    pub fn add_ent(&mut self, ent: Ent) {
        self.ents.push(ent);
    }
    pub fn draw(&mut self) {
        for t in self.tiles.iter().clone() {
            t.draw(self);
        }
    }
    pub fn draw_normals(&mut self) {
        for t in self.tiles.iter() {
            t.draw_normals(self);
        }
    }
    pub fn get_x(&self) -> f32 {
        self.x.clone()
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn get_tile(&mut self, index: usize) -> TileBlock {
        self.tiles[index]
    }
    pub fn pos_add(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }
}
