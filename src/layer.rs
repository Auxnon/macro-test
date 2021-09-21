use crate::Ent;
use crate::TileBlock;

pub struct Layer<'a> {
    x: f32,
    y: f32,
    scale: f32,
    tiles: Vec<TileBlock>,
    ents: Vec<Ent<'a>>,
}

impl<'a> Layer<'a> {
    /*pub fn new(scale: f32) -> Layer {
        //let ar = [[Tile { id: 10, x: 0, y: 0 }; 16]; 16];

        Layer::new(scale, 0, 0)
    }*/
    pub fn new(scale: f32, x: f32, y: f32) -> Layer<'a> {
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
    pub fn add_ent(&mut self, ent: Ent<'a>) {
        self.ents.push(ent);
    }
    pub fn draw(&mut self, delta: f32, tick: bool) {
        for t in self.tiles.iter().clone() {
            t.draw(self);
        }
        for e in self.ents.iter_mut() {
            e.draw(delta, tick, false);
        }
    }
    pub fn draw_normals(&mut self, delta: f32, tick: bool) {
        for t in self.tiles.iter() {
            t.draw_normals(self);
        }
        for e in self.ents.iter_mut() {
            e.draw(delta, tick, true);
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
