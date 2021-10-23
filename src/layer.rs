use crate::Ent;
use crate::Tile;
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
    pub fn run(&mut self, delta: f32) {
        for e in self.ents.iter_mut() {
            e.run(delta);
            e.grounded = false;
            e.edge_left = false;
            e.edge_right = false;
            if self.tiles.len() > 0 {
                //let mut block = self.tiles[0];

                let v = self.tiles[0].get_in_range(e.pos.x, e.pos.y, e.get_width(), e.get_height());
                for tile in v {
                    //println!("at {} {}", tile.x, tile.y);
                    tile.set_flag(true);
                    if tile.id == 2 {
                        check(e, tile)
                    }
                }
            }
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
        for t in self.tiles.iter_mut() {
            t.draw(self.x, self.y);
        }
        for e in self.ents.iter_mut() {
            e.draw(delta, tick, false);
        }
    }
    pub fn draw_normals(&mut self, delta: f32, tick: bool) {
        for t in self.tiles.iter_mut() {
            t.draw_normals(self.x, self.y);
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

fn check(e: &mut Ent, tile: &mut Tile) {
    let tx = tile.x as f32 * 16.;
    let ty = tile.y as f32 * 16.;

    if e.pos.x < tx + 16. && e.pos.x + e.get_width() > tx {
        let cx = (e.pos.x + e.get_width() / 2.);
        let cy = (e.pos.y + e.get_height() / 2.);
        let deltax = (tx + 8.) - cx;
        let deltay = (ty + 8.) - cy;

        let left = (e.pos.x + e.get_width()) - tx;
        let right = e.pos.x - (tx + 16.);
        let dx = if deltax < 0. { left } else { right };

        // let dy1 = (e.pos.y - e.get_height()) - ty;
        // let dy2 = (e.pos.y + e.get_height()) - (ty + 16.);
        // let dy = if deltay < 0. { dy1 } else { dy2 };

        // let top_diff=(e.pos.y+e.get_height())-ty;
        // let bot_diff=e.pos.y-(ty+16.);
        // let left_diff=(e.pos.x+e.get_width())-tx;
        // let right_diff=e.pos.x-(tx+16.);
        // if left_diff<right_diff{
        //     if
        // }
        // let top=top_diff<bot_diff;
        fn p(e: &mut Ent, left: f32, right: f32) {
            println!("x {} y{} left {} right {}", e.pos.x, e.pos.y, left, right);
        }

        if deltax.abs() > deltay.abs() {
            if dx < 0. {
                //p(e, left, right);
                e.pos.x = tx - e.get_width();
                e.vel.x = 0.;
                e.edge_left = true;
                //-e.vel.x.abs() * 0.5;
            } else {
                e.pos.x = tx + 16.;
                e.vel.x = 0.;
                e.edge_right = true;
                //e.vel.x.abs() * 0.5;
            }
        } else {
            if deltay < 0. {
                //ent e colliding from the top in our current coordinate system
                e.pos.y = ty + 16.;
                e.vel.y = 0.;

                //-e.vel.y.abs() * 0.5;
                // p(e);
                //println!("top tile {} or {} and {} won", dy1, dy2, dy);
            } else {
                e.pos.y = ty - e.get_height() - 0.001;
                e.vel.y = 0.;
                e.grounded = true;

                //e.vel.y = e.vel.y.abs() * 0.5;
                // p(e);
                //println!("bottom tile {} / {}", dx, dy);
            }
        }
    }
    /*
    let ox = (e.pos.x / 16.).floor() * 16.;
    let oy = (e.pos.y / 16.).floor() * 16.;
    let xx = e.pos.x - ox;
    let yy = e.pos.y - oy;
    //println!("og {} {}", xx, yy);
    if xx.abs() > yy.abs() {
        if xx < 0. {
            e.pos.x = ox;
            e.vel.x = 0.;
            if e.vel.x > 0. {}
        } else if xx > 0. {
            e.pos.x = ox + 16.;
            e.vel.x = 0.;
            if e.vel.x < 0. {
                e.vel.x = 0.
            }
        }
    } else {
        if yy < 0. {
            // e.pos.y = oy - 1.;
            // e.vel.y = 0.;
            // if e.vel.y < 0. {}
        } else if yy > 0. {
            e.pos.y = oy - 1.;
            e.vel.y = 0.;
            if e.vel.y > 0. {
                e.vel.y = 0.
            }
            e.grounded = true;
        }
    }*/
}
