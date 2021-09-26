use gltf::buffer::Data as BufferData;

use gltf::image::{Data as ImageData, Source};
use gltf::json::extensions::mesh::*;
use gltf::json::extensions::scene::*;
use gltf::mesh::util::*;
use gltf::mesh::*;
use gltf::scene::Node;
use gltf::{Document, Gltf, Mesh, Primitive, Scene};
use itertools::{izip, Itertools};

//use gltf_importer::import;
use macroquad::prelude::*;
mod controls;
mod entity;
mod global;
mod image_helper;
mod layer;
mod menu;
mod shader_loader;
mod tile;

use entity::{Ent, EntFactory};
use global::Global;
use layer::Layer;
use std::collections::HashMap;
use std::process::exit;
use tile::TileBlock;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Kiwi"),
        window_width: 1280,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf())]
async fn main() {
    // 320 x 192

    let mut ar: [[u8; 20]; 12] = [[0; 20]; 12];

    let level_template: Image = load_image("assets/level_template.png").await.unwrap();
    for i in 0..20 {
        for j in 0..12 {
            let c = level_template.get_image_data()[((i + j * 20) as usize)]; //value
            if c[0] == 0 && c[1] == 0 && c[2] == 0 {
                //println!("black");
                ar[j][i] = 2;
            } else {
                //println!("white");
                ar[j][i] = 3;
            }
        }
    }

    let ent_factory = EntFactory::new().await;
    /*****
     * Set palette bloom and shading values for our shader
     */
    let color_img: Image = load_image("assets/colors.png").await.unwrap();
    let cc = color_img.get_image_data()[((5) as usize)]; //value
    let mut lookup_image =
        Image::gen_image_color(256, 32, Color::from_rgba(cc[0], cc[1], cc[2], 255));
    for i in 0..32 {
        let c = color_img.get_image_data()[((i + 32) as usize)]; //value
        let l = color_img.get_image_data()[((i) as usize)]; //low
        let h = color_img.get_image_data()[((i + 64) as usize)]; //high
        let [x, y, z] = [c[0] / 16, c[1] / 16, c[2] / 16];
        lookup_image.set_pixel(
            (x + (z * 16)) as u32,
            y as u32,
            Color::from_rgba(l[0], l[1], l[2], l[3]),
        );
        lookup_image.set_pixel(
            (x + (z * 16)) as u32,
            (y + 16) as u32,
            Color::from_rgba(h[0], h[1], h[2], h[3]),
        );
    }

    let color_lookup = Texture2D::from_image(&lookup_image);
    color_lookup.set_filter(FilterMode::Nearest);

    /***
     * END
     */

    let mut globals: Global = Default::default();
    let tile_template: tile::TileTemplate = tile::create_template("assets/wood").await;
    let mut tiles: TileBlock = TileBlock::new(0, 0, tile_template, ar);
    /***
     * Test Two
     */

    let mut layer: Layer = Layer::new(1., 0., 576.); //573
                                                     //tiles.pos_add(20, 0);
    layer.add_tile(tiles);
    layer.add_ent(ent_factory.create_ent("birb-npc"));

    /***
     * END Test Two
     */

    let iwidth = (screen_width() as u16) / 4;
    let iheight = (screen_height() as u16) / 4;

    let img_pull = get_screen_data();
    let render_pass_first = Texture2D::from_image(&img_pull);
    render_pass_first.set_filter(FilterMode::Nearest);

    let render_pass_second = Texture2D::from_image(&img_pull);
    render_pass_second.set_filter(FilterMode::Nearest);

    let screen_material = load_material(
        &std::fs::read_to_string("src/shader.vert").expect("uh oh bad glsl file"),
        &std::fs::read_to_string("src/shader.frag").expect("uh oh bad glsl file"),
        MaterialParams {
            uniforms: vec![
                ("Center".to_owned(), UniformType::Float2),
                ("ray".to_owned(), UniformType::Float2),
                ("resolution".to_owned(), UniformType::Float2),
                ("ratio".to_owned(), UniformType::Float1),
                ("time".to_owned(), UniformType::Float1),
            ],
            textures: vec![
                //"Texture".to_owned() // this one is defined by Macroquad. assign other manually if needed.
                "normals".to_owned(),
                "albedo".to_owned(),
                "remap".to_owned(),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    let mut incr_time: f32 = 0.;

    let mut last_step_time = 0.;
    let mut last_real_time = 0.;

    screen_material.set_texture("remap", color_lookup);

    let mut last_sw = screen_width();
    let mut last_sh = screen_height();

    // render_pass_first.update(&get_screen_data());
    // render_pass_second.update(&get_screen_data());
    loop {
        let mw = screen_width() / 2.;
        let mh = screen_height() / 2.;
        let ir = screen_width() / 320.;
        let pixHeight = screen_height() / ir;

        let lens_center = mouse_position();

        let delta_point = (
            (lens_center.0 / screen_width()),
            (lens_center.1 / screen_height()),
        );
        let r = (delta_point.0 * delta_point.0 + delta_point.1 * delta_point.1).sqrt();

        screen_material.set_uniform("Center", lens_center);
        screen_material.set_uniform(
            "ray",
            (2. * (delta_point.0 - 0.5), 2. * (delta_point.1 - 0.5)),
        );

        screen_material.set_uniform("resolution", (320. as f32, pixHeight as f32));
        screen_material.set_uniform("ratio", ir);
        screen_material.set_uniform("time", incr_time);

        let real_time = get_time();

        let tick = if real_time > last_step_time + 0.25 {
            true
        } else {
            false
        };
        incr_time += (real_time / 1000.) as f32;
        if incr_time > 1. {
            incr_time -= 1.;
        }
        let delta = real_time - last_real_time;
        //last_real_time = real_time;

        if last_sw == screen_width() && last_sh == screen_height() {
            /* ======== Larry 3D

             _   _                            _
            | \ | |                          | |
            |  \| | ___  _ __ _ __ ___   __ _| |___
            | . ` |/ _ \| '__| '_ ` _ \ / _` | / __|
            | |\  | (_) | |  | | | | | | (_| | \__ \
            |_| \_|\___/|_|  |_| |_| |_|\__,_|_|___/
                    =========*/
            //tiles.pos_add(1, 0);
            //layer.pos_add(0., 0.1);
            //layer.get_tile(0).pos_add(1, 0);
            layer.draw_normals(delta as f32, tick);
            render_pass_first.update(&get_screen_data()); //dump our screen texture to our render_pass_first variable
            screen_material.set_texture("normals", render_pass_first); //send this screen capture to our shader
            clear_background(BLACK);
            /* ========
                      _ _              _
                /\   | | |            | |
               /  \  | | |__   ___  __| | ___
              / /\ \ | | '_ \ / _ \/ _` |/ _ \
             / ____ \| | |_) |  __/ (_| | (_) |
            /_/    \_\_|_.__/ \___|\__,_|\___/

                    =========*/

            layer.draw(delta as f32, tick);

            //wrap up pass
            render_pass_second.update(&get_screen_data());
            screen_material.set_texture("albedo", render_pass_second); //send this screen capture to our shader
            clear_background(WHITE);
            //done
        }

        /*
         _    _                     _
        | |  | |                   | |
        | |  | |_ __  ___  ___ __ _| | ___
        | |  | | '_ \/ __|/ __/ _` | |/ _ \
        | |__| | |_) \__ \ (_| (_| | |  __/
         \____/| .__/|___/\___\__,_|_|\___|
               | |
               |_|

                */
        set_default_camera();
        gl_use_material(screen_material);
        draw_rectangle(0., 0., screen_width(), screen_height(), RED);

        gl_use_default_material();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            layer.remove_tile(0);
            layer.add_tile(TileBlock::new(20, 20, tile_template, ar));
        }

        controls::cycle(&mut globals);
        if is_mouse_button_pressed(MouseButton::Left) {
            let t = mouse_position_local();
            let xx = ((t.x + 1.) / 2.) as u16;

            let v = screen_height() * (t.y + 1.) / (2. * ir);
            let scaled = ir * 192.;
            let half_offset = (screen_height() - scaled) / 2.;
            println!("half {} v {}", half_offset, v);
            if v > half_offset && v < (screen_height() - half_offset) {
                let yy = (v - half_offset) as u16 / 16;
                layer.get_tile(0).set(10, xx, yy);
                println!("x {} y {}", xx, yy);
            } else {
                println!("nope x {} v {}", xx, v);
            }
        }

        last_sw = screen_width();
        last_sh = screen_height();
        next_frame().await
    }
    println!("complete");
    exit(0);
}

fn drawAlbedo() {}

fn drawNormals() {}
