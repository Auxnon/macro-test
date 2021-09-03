use macroquad::prelude::*;

pub fn set(n: u8, x: u16, y: u16) {
    //draw
}

#[derive(Copy, Clone)]
pub struct Tile {
    id: u8,
    x: u16,
    y: u16,
} /*
  impl{
      fn new()->Tile{
          Tile{}
      }

  }*/
pub struct TileBlock {
    x: u32,
    y: u32,
    array: [[Tile; 20]; 12],
    texture: Texture2D,
    normals: Texture2D,
}
fn int_to_tile(tiles: [[u8; 20]; 12]) -> [[Tile; 20]; 12] {
    let mut ar = [[Tile { id: 10, x: 0, y: 0 }; 20]; 12];
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
impl TileBlock {
    pub async fn new(x: u32, y: u32, texture: &str, tiles: [[u8; 20]; 12]) -> TileBlock {
        //let ar = [[Tile { id: 10, x: 0, y: 0 }; 16]; 16];
        let out = [texture, ".png"].join("");
        let nout = [texture, "_n.png"].join("");
        let textur: &str = &*out;
        let normal: &str = &*nout; //explicit reborrowing, just to covnert our dynamic String to a static str, wow!
                                   //s.push_str("_n");
        TileBlock {
            x,
            y,
            array: int_to_tile(tiles),
            texture: load_texture(textur).await.unwrap(),
            normals: load_texture(normal).await.unwrap(),
        }
    }
    pub fn set(&mut self, id: u8, x: u16, y: u16) {
        // let t = Tile { id, x, y };
        self.array[y as usize][x as usize].id = id;
    }
    pub fn draw_normals(&mut self) {
        for i in 0..self.array[0].len() {
            for j in 0..self.array.len() {
                //self.array[i][j]
                //draw_texture_ex()
                let id = self.array[j][i].id;
                let x = (id % 11) as f32;
                let y = (id / 11) as f32;
                draw_texture_ex(
                    self.normals,
                    (i * 16) as f32,
                    384. + (j * 16) as f32,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(x * 16., y * 16., 16., 16.)),
                        //flip_x: dir,
                        ..Default::default()
                    },
                );
            }
        }
    }
    pub fn draw(&mut self) {
        for i in 0..self.array[0].len() {
            for j in 0..self.array.len() {
                //self.array[i][j]
                //draw_texture_ex()
                let id = self.array[j][i].id;
                let x = (id % 11) as f32;
                let y = (id / 11) as f32;
                draw_texture_ex(
                    self.texture,
                    (i * 16) as f32,
                    384. + (j * 16) as f32,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(x * 16., y * 16., 16., 16.)),
                        //flip_x: dir,
                        ..Default::default()
                    },
                );
            }
        }
    }
}
