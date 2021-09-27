use macroquad::prelude::*;

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
        countx: (t.width() / 16.) as u32,
        county: (t.height() / 16.) as u32,
        texture: t,
        normals: n,
    }
}

#[derive(Copy, Clone)]
pub struct TileTemplate {
    texture: Texture2D,
    normals: Texture2D,
    countx: u32,
    county: u32,
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
#[derive(Copy, Clone)]
pub struct TileBlock {
    x: u32,
    y: u32,
    array: [[Tile; 20]; 12],
    template: TileTemplate,
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
    pub fn new(x: u32, y: u32, template: TileTemplate, tiles: [[u8; 20]; 12]) -> TileBlock {
        //let ar = [[Tile { id: 10, x: 0, y: 0 }; 16]; 16];
        TileBlock {
            x,
            y,
            array: int_to_tile(tiles),
            template,
        }
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
    pub fn draw_normals(&self, layer: &Layer) {
        self._draw(layer, self.template.normals);
    }
    pub fn draw(&self, layer: &Layer) {
        self._draw(layer, self.template.texture);
    }
    fn _draw(&self, layer: &Layer, texture: Texture2D) {
        let ox = layer.get_x() + self.x as f32;
        let oy = layer.get_y() + self.y as f32;
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
