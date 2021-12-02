use gltf::buffer::Data as BufferData;

use gltf::image::{Data as ImageData, Source};
use gltf::json::extensions::mesh::{self, *};
use gltf::json::extensions::scene::*;
use gltf::mesh::util::*;
use gltf::scene::Node;
use gltf::{mesh::*, Texture};
use gltf::{Document, Gltf, Mesh, Primitive, Scene};
use itertools::{izip, Itertools};

//use gltf_importer::import;
use macroquad::prelude::*;
mod controls;
mod ent;
mod ent_factory;
mod global;
mod image_helper;
mod layer;
mod logic;
mod lua_define;
mod lua_ent;
mod menu;
mod shader_loader;
mod three_loader;
mod three_test;
mod tile;
mod tile_factory;

use ent::Ent;
use ent_factory::EntFactory;
use global::Global;
use layer::Layer;
use logic::get_logic;
use lua_ent::LuaEnt;
use mlua::{Lua, Scope};

use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use tile::Tile;
use tile::TileBlock;
use tile_factory::init_tiles;

use crate::lua_define::{scope_test, LuaCore};

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
    let ent_factory = EntFactory::new().await;
    let lua_core = LuaCore::new();
    let mut meshes_vec: Vec<Ent> = vec![];
    let mut meshes = Rc::new(RefCell::new(meshes_vec));

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
    init_tiles();

    /*****
     * Set palette bloom and shading values for our shader
     */
    let color_img: Image = load_image("assets/colors.png").await.unwrap();
    let grass_test: Texture2D = load_texture("assets/sprites/grass_down.png").await.unwrap();
    grass_test.set_filter(FilterMode::Nearest);
    let guy_test: Texture2D = load_texture("assets/sprites/guy-test.png").await.unwrap();
    guy_test.set_filter(FilterMode::Nearest);

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

    let mut globals: Global = Default::default();
    let tile_template: tile::TileTemplate = tile::create_template("assets/wood").await;
    let mut tiles: TileBlock = TileBlock::new(0, 0, &tile_template, ar);

    let scope_result: Result<u32, mlua::Error> = lua_core.lua.scope(|scope: &Scope| {
        //lua_core.init(scope, &ent_factory, Rc::clone(&meshes));

        scope_test(scope, &ent_factory, &lua_core, Rc::clone(&meshes));
        // let make_ent = |str: String| {
        //     let ee = Rc::clone(&ent_factory);
        //     ee.create_ent(&str, &lua_core)
        // };
        macro_rules! make_ent {
            ($a:expr) => {
                (ent_factory.create_ent($a, &lua_core));
            };
        }

        let house = make_ent!(&"house".to_string()); //ent_factory.create_ent(&"house".to_string(), &lua_core);
        meshes.borrow_mut().push(house);
        for i in 1..6 {
            let mut dude = make_ent!(&"dude".to_string());
            dude.accessory = Some(ent_factory.get_schema("axe"));
            dude.pos.x = rand::gen_range(-2., 2.);
            dude.pos.z = rand::gen_range(-2., 2.);
            meshes.borrow_mut().push(dude);
        }

        let mut maker = make_ent!(&"maker".to_string());
        maker.pos.x = -3.;
        maker.pos.z = -3.;
        meshes.borrow_mut().push(maker);

        /***
         * Test Two
         */
        let mut layer: Layer = Layer::new(1., 0., 0.);
        layer.add_tile(tiles);

        let mut tiles3: TileBlock = TileBlock::new(0, 0, &tile_template, ar);
        let mut layer3: Layer = Layer::new(1., 0., 0.);
        layer3.add_tile(tiles3);

        let mut player = make_ent!(&"kiwi".to_string());
        player.set_xy(16. * 14., 16. * 6.);
        //layer.add_ent(player);

        // for i in 0..10 {
        //     let mut player2 = ent_factory.create_ent("kiwi");
        //     player2.set_xy(16. * (8. + i as f32), 16. * 6.);
        //     layer.add_ent(player2);
        // }

        // let mut kp = ent_factory.create_ent("kiwi-portrait");
        // kp.set_xy(16. * 14., 16. * 6.);
        // layer.add_ent(kp);

        let mut npc = make_ent!(&"birb-npc".to_string());
        npc.set_xy(16. * 12., 16. * 6.);
        //layer.add_ent(npc);

        /***
         * END Test Two
         */

        let iwidth = (screen_width() as u16) / 4;
        let iheight = (screen_height() as u16) / 4;

        let img_pull = get_screen_data();
        /*Image {
            width: 320,
            height: 192,
            bytes: vec![],
        };*/
        let mut render_pass_first = Texture2D::from_image(&img_pull);
        render_pass_first.set_filter(FilterMode::Nearest);

        let mut render_pass_second = Texture2D::from_image(&img_pull);
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
            game_loop(
                screen_material,
                incr_time,
                last_step_time,
                last_real_time,
                render_pass_first,
                render_pass_second,
                grass_test,
                guy_test,
                &mut globals,
                Rc::clone(&meshes),
                &mut layer,
                &mut layer3,
                last_sw,
                last_sh,
            );
        }
    });

    if scope_result.is_err() {
        println!("yikes, lua failed real hard, quitting");
        //exit(0);
    }
    println!("complete");
    exit(0);
}
async fn game_loop<'a>(
    screen_material: Material,
    mut incr_time: f32,
    mut last_step_time: f64,
    mut last_real_time: f64,
    render_pass_first: Texture2D,
    render_pass_second: Texture2D,
    grass_test: Texture2D,
    guy_test: Texture2D,
    globals: &mut Global,
    meshes: Rc<RefCell<Vec<Ent<'_>>>>,
    layer: &mut Layer<'a>,
    layer3: &mut Layer<'a>,
    mut last_sw: f32,
    mut last_sh: f32,
) {
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

    let tick = if real_time > last_step_time + 0.1 {
        last_step_time = real_time;
        true
    } else {
        false
    };
    incr_time = (real_time / 10.) as f32;
    incr_time = incr_time % 1.;

    let delta = real_time - last_real_time;
    //last_real_time = real_time;

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
    unsafe {
        macroquad::window::get_internal_gl().flush();
    }
    render_pass_first.grab_screen(); //dump our screen texture to our render_pass_first variable
    screen_material.set_texture("normals", render_pass_first); //send this screen capture to our shader
    clear_background(Color::new(0.803, 0.945, 0.976, 1.));
    /* ========
              _ _              _
        /\   | | |            | |
       /  \  | | |__   ___  __| | ___
      / /\ \ | | '_ \ / _ \/ _` |/ _ \
     / ____ \| | |_) |  __/ (_| | (_) |
    /_/    \_\_|_.__/ \___|\__,_|\___/

            =========*/
    three_test::render(
        delta as f32,
        incr_time,
        grass_test,
        guy_test,
        Rc::clone(&meshes),
        tick,
        layer3,
        globals,
    );
    let globalOut: &mut Global = controls::cycle(globals);
    layer.draw(delta as f32, tick);

    //wrap up pass
    unsafe {
        macroquad::window::get_internal_gl().flush();
    }
    render_pass_second.grab_screen();
    screen_material.set_texture("albedo", render_pass_second); //send this screen capture to our shader
                                                               //clear_background(WHITE);
                                                               //done

    if last_sw == screen_width() && last_sh == screen_height() {}

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
    draw_texture_ex(
        render_pass_second,
        0.,
        0., //+ 384.,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(0., screen_height() - 192., 320., 192.)),
            flip_y: true,
            dest_size: Some(Vec2::new(screen_width(), screen_height())),
            ..Default::default()
        },
    );
    //
    //draw_rectangle(128., 0., screen_width(), screen_height(), RED);

    gl_use_default_material();

    // if is_key_pressed(KeyCode::Escape) {
    //     break;
    // }

    layer.run((delta) as f32);

    last_sw = screen_width();
    last_sh = screen_height();
    last_real_time = real_time;
    next_frame();
}
/*
fn get_screen_data_custom() -> Image {
    unsafe {
        macroquad::window::get_internal_gl().flush();
    }

    let context = macroquad::

    let texture = Texture2D::from_miniquad_texture(miniquad::Texture::new_render_texture(
        &mut context.quad_context,
        miniquad::TextureParams {
            width: context.screen_width as _,
            height: context.screen_height as _,
            ..Default::default()
        },
    ));

    texture.grab_screen();

    texture.get_texture_data()
}*/
