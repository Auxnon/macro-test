use gltf::buffer::Data as BufferData;

use gltf::image::{Data as ImageData, Source};
use gltf::json::extensions::mesh::*;
use gltf::json::extensions::scene::*;
use gltf::mesh::util::*;
use gltf::mesh::*;
use gltf::scene::Node;
use gltf::{Document, Gltf, Mesh, Primitive, Scene};

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
use tile::TileBlock;

#[macroquad::main("Kiwi")]
async fn main() {
    let ar: [[u8; 20]; 12] = [
        [8, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8],
        [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8],
    ];
    let ent_factory = EntFactory::new().await;

    /*****
     * Test One
     */
    let color_img: Image = load_image("assets/colors.png").await.unwrap();
    let cc = color_img.get_image_data()[((5) as usize)]; //value
    let mut lookup_image =
        Image::gen_image_color(256, 32, Color::from_rgba(cc[0], cc[1], cc[2], 255));
    //let mut COLOR: HashMap<String, [[u8; 4]; 2]> = HashMap::new();
    //let array2: [[[u8; 255]; 255]; 255]; // = [[[0; 255]; 255]; 255];
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
        //let together = format!("{}{}{}", c[0], c[1], c[2]);
        //println!("big {}", c[0] & c[1] & c[2]);
        // array[(c[0] & c[1] & c[2]) as usize] = [
        //     color_img.get_image_data()[((i) as usize)],
        //     color_img.get_image_data()[((i + 64) as usize)],
        // ];
        /*COLOR.insert(
            together,
            [
                color_img.get_image_data()[((i) as usize)],
                color_img.get_image_data()[((i + 64) as usize)],
            ],
        );*/
        //println!("{} rgb {} {} {} {}", i, c[0], c[1], c[2], c[3]);
        //println!("ci ${} ${}", (x + z * 16) as f32 / 256., y as f32 / 32.);
    }

    let color_lookup = Texture2D::from_image(&lookup_image);
    color_lookup.set_filter(FilterMode::Nearest);

    /***
     * END Test One
     */

    let mut globals: Global = Default::default();
    let tile_template: tile::TileTemplate = tile::create_template("assets/tiles").await;
    let mut tiles: TileBlock = TileBlock::new(0, 0, tile_template, ar);
    /***
     * Test Two
     */

    let mut layer: Layer = Layer::new(1., 0., 0.);
    tiles.pos_add(20, 0);
    layer.add_tile(tiles);
    layer.add_ent(ent_factory.create_ent("birb-npc"));

    /***
     * END Test Two
     */

    /*
    Test Three
    */
    let (nodes, buffers, image_data) = gltf::import("assets/console.gltf").unwrap();
    //let (gltf, buffers) = import().unwrap();
    //let mut take = gltf.accessors();
    //let buffer_data = buffers.take();
    /*for row in vec {
        println!("========");
        println!("========");
        print!("v");
        for i in row {
            print!("{},", i);
        }
    }*/

    //let json=gltf.document.into_json();
    //json.meshes[0].primitives[0].indices
    //gltf.document.buffers().for_each(f: F)
    /*let mat = take.next();
    match mat {
        Some(o) => {
            println!("mesh {:?}", o)
        }
        None => {
            println!("mesh naw");
        }
    };
    let mat2 = take.next();
    match mat2 {
        Some(o) => {
            println!("mesh2 {:?}", o)
        }
        None => {
            println!("mesh2 naw");
        }
    };*/
    for mesh in nodes.meshes() {
        //draw_mesh();
        //let m=Mesh{}

        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            let faces: Vec<u16> = match reader.read_indices().unwrap() {
                ReadIndices::U8(itr) => itr.map(|i| i as u16).collect(),
                ReadIndices::U16(itr) => itr.collect(),
                ReadIndices::U32(itr) => itr.map(|i| i as u16).collect(),
            };
            for f in faces {
                println!("- face #{:?}", f);
            }
            //let verts_interleaved = izip!(
            let m = (
                reader.read_positions().unwrap(),
                reader.read_normals().unwrap(),
                //reader.read_colors(0).unwrap().into_rgb_f32().into_iter(),
                reader.read_tex_coords(0).unwrap().into_f32(),
            );

            if let (Some(verts), Some(uvs)) = (
                reader.read_positions().map(|v| v),
                reader.read_positions().map(|u| u),
            ) {
                //let p = Vertex{pos:verts,uv:uvs}
            }

            //);
            pub struct Vertex {
                pos: [f32; 3],
                uv: [f32; 2],
                color: [u8; 4],
            }

            /*let verts = verts_interleaved
            .map(|(pos, norm, uv)| Vertex {
                x: match pos.get(0) {
                    Some(p) => *p,
                    _ => 0.0,
                },
                y: match pos.get(1) {
                    Some(p) => *p,
                    _ => 0.0,
                },
                z: match pos.get(2) {
                    Some(p) => *p,
                    _ => 0.0,
                },
                nx: match norm.get(0) {
                    Some(n) => *n,
                    _ => 0.0,
                },
                ny: match norm.get(1) {
                    Some(n) => *n,
                    _ => 0.0,
                },
                nz: match norm.get(2) {
                    Some(n) => *n,
                    _ => 0.0,
                },
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
                u: match uv.get(0) {
                    Some(u) => match u {
                        //u if *u > 1.0 => u.fract(),
                        //u if *u < 0.0 => u.fract() + 1.0,
                        _ => *u,
                    },
                    _ => 0.0,
                },
                v: match uv.get(1) {
                    Some(v) => match v {
                        //v if *v > 1.0 => v.fract(),
                        //v if *v < 0.0 => v.fract() + 1.0,
                        _ => *v,
                    },
                    _ => 0.0,
                },
            })
            .collect::<Vec<Vertex>>();*/
            /*
            println!("- Primitive #{}", primitive.index());
            println!(
                "bounds {} {}",
                primitive.bounding_box().max[0],
                primitive.bounding_box().min[0]
            );
            for (semantic, _) in primitive.attributes() {
                print!("-- {:?}", semantic);
                //let reader = primitive.reader()
                let accessor = primitive.get(&semantic).unwrap();
                //gltf_utils::Positions::new(accessor, primitive);
                //accessor.offset()
                println!("-- {}", accessor.count());
            }*/
        }
        //mesh.as_json().primitives
    }
    //draw_mesh(mesh: &Mesh);
    //draw_mesh(mesh: &Mesh)

    /***
     *  End test Three
     */

    let iwidth = (screen_width() as u16) / 4;
    let iheight = (screen_height() as u16) / 4;

    let birb: Texture2D = load_texture("assets/birb.png").await.unwrap();
    let birb_n_img: Image = load_image("assets/birb_n.png").await.unwrap();
    let birb_n: Texture2D = Texture2D::from_image(&birb_n_img); //load_texture("assets/birbo_n.png").await.unwrap();
    let immm = image_helper::flip(&birb_n_img, 0);
    println!("returned ${}", immm.get_image_data()[((13) as usize)][2]);
    let birb_nf: Texture2D = Texture2D::from_image(&immm);

    let mut image = Image::gen_image_color(iwidth, iheight, WHITE);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let im = get_screen_data();
    let render_pass_first = Texture2D::from_image(&im);
    render_pass_first.set_filter(FilterMode::Nearest);

    let render_pass_second = Texture2D::from_image(&im);
    render_pass_second.set_filter(FilterMode::Nearest);

    let birbImg: Image = load_image("assets/kiwi.png").await.unwrap();
    birb.set_filter(FilterMode::Nearest);
    let chess: Texture2D = load_texture("assets/chess.png").await.unwrap();
    chess.set_filter(FilterMode::Nearest);
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

    let mut tick: u8 = 0;
    let mut array: Vec<(f32, f32, bool)> = Vec::new();
    let mut array_is_dirty: bool = false;
    array.push((3., 128., true));

    let mut iter: u32 = 0;

    let mut time: f32 = 0.;

    loop {
        let mw = screen_width() / 2.;
        let mh = screen_height() / 2.;
        let ir = screen_width() / 320.;
        let pixHeight = screen_height() / ir;

        /*tick += 1;
        if tick >= 6 {
            tick = 0;
            anim += 1;
            if anim >= max {
                anim = 0;
            }
        }*/

        let lens_center = mouse_position();

        /*
        clear_background(BLACK);
        for i in 0..32 {
            for j in 0..32 {
                let n = 64; //(if lens_center.1 >iheight {iheight} else {lens_center.1}) as u16;
                image.get_image_data_mut()
                    [((i + n) * iwidth + j + lens_center.0 as u16) as usize] =
                    birbImg.get_image_data()[(i * 32 + j) as usize];
            }
        }*/

        let delta = (
            (lens_center.0 / screen_width()),
            (lens_center.1 / screen_height()),
        );
        let r = (delta.0 * delta.0 + delta.1 * delta.1).sqrt();

        screen_material.set_uniform("Center", lens_center);
        screen_material.set_uniform("ray", (2. * (delta.0 - 0.5), 2. * (delta.1 - 0.5)));
        //println!("ray {} {}", 2. * (delta.0 - 0.5), 2. * (delta.1 - 0.5));
        screen_material.set_uniform("resolution", (320. as f32, pixHeight as f32));
        screen_material.set_uniform("ratio", ir);
        screen_material.set_uniform("time", time);
        time += 0.01;
        if time >= 1. {
            time = 0.;
        }
        /* ======== Larry 3D

         _   _                            _
        | \ | |                          | |
        |  \| | ___  _ __ _ __ ___   __ _| |___
        | . ` |/ _ \| '__| '_ ` _ \ / _` | / __|
        | |\  | (_) | |  | | | | | | (_| | \__ \
        |_| \_|\___/|_|  |_| |_| |_|\__,_|_|___/
                =========*/
        //tiles.pos_add(1, 0);
        //layer.pos_add(0., 1.);
        layer.get_tile(0).pos_add(1, 0);
        layer.draw_normals();

        //set_default_camera();

        texture.update(&image);

        render_pass_first.update(&get_screen_data()); //dump our screen texture to our render_pass_first variable
        screen_material.set_texture("normals", render_pass_first); //send this screen capture to our shader
        screen_material.set_texture("remap", color_lookup); //send this screen capture to our shader
        clear_background(WHITE);

        //draw_texture(textureFirst, 64.,64.,WHITE);
        //get_active_render_pass();
        //texture()
        //draw_mode()

        /* ========
                  _ _              _
            /\   | | |            | |
           /  \  | | |__   ___  __| | ___
          / /\ \ | | '_ \ / _ \/ _` |/ _ \
         / ____ \| | |_) |  __/ (_| | (_) |
        /_/    \_\_|_.__/ \___|\__,_|\___/

                =========*/

        /*if true {
            set_camera(&Camera3D {
                //position: vec3(0.001, 1., 0.),
                position: vec3(1., 10., 0.),
                up: vec3(0., 1., 0.),
                target: vec3(0., 0., 0.),
                ..Default::default()
            });
        }*/

        layer.draw();
        /*for i in 0..array.len() {
            //let dir=array[i].2;
            draw_texture_ex(
                birb,
                array[i].0 - 16.,
                array[i].1 - 16. + 384., //????
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new((anim as f32) * 32., 0., 32., 32.)),
                    //dest_size: Some(vec2(32., 32.)),
                    flip_x: array[i].2,
                    ..Default::default()
                },
            );
        }*/

        draw_cube(
            Vec3::new(time * 10., 0., 0.),
            Vec3::new(10., 10., 10.),
            render_pass_first,
            WHITE,
        );

        /*draw_cube(
            Vec3::new(100., 200., time * 200. - 100.),
            Vec3::new(200., 200., 256.),
            birb_n,
            PURPLE,
        );*/
        //draw_rectangle(100., time * 200., 200., 200., RED);

        //wrap up pass
        render_pass_second.update(&get_screen_data());
        screen_material.set_texture("albedo", render_pass_second); //send this screen capture to our shader
        clear_background(WHITE);
        //done

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
        //plain texture render?
        /* draw_texture_ex(
            render_pass_second,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(0., 0., 320., 180.)),
                dest_size: Some(vec2(screen_width(), screen_height())),

                ..Default::default()
            },
        );*/
        //draw_circle(screen_width()/2., screen_height()/2., 350.0, RED);
        gl_use_default_material();
        let p = ((time) - 0.5) * 2000.;
        if false {
            set_camera(&Camera3D {
                //position: vec3(0.001, 1., 0.),
                position: vec3(2000., 2000., 0.),
                up: vec3(0., 1., 0.),
                target: vec3(0., 0., 0.),
                ..Default::default()
            });
        }
        //draw_rectangle(0., 0., screen_width(), screen_height(), GREEN);

        //draw_cube(Vec3::new(0., 0., 0.), Vec3::new(1., 1., 1.), birb_n, WHITE);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            layer.remove_tile(0);
            layer.add_tile(TileBlock::new(20, 20, tile_template, ar));
            if iter > 192 {
                iter = 0;
            }
            array.push((128., iter as f32, true)); //and::gen_range(20., 160.)
        }

        //dump our dead bois safely outside the entity loop array
        if array_is_dirty {
            //info!("dirty");
            for i in (0..array.len()).rev() {
                if array[i].0 == -99. {
                    //info!("dumped and now array now size {}",array.len());
                    array.remove(i);
                }
            }

            array_is_dirty = false;
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

            //let y1 = ((t.y + 1. / ir) * 6. + 6.) as u16;
            //let yy = clamp(y1, 0, 11);
            //tiles.set(10, xx, yy)
        }

        next_frame().await
    }
    println!("complete");
}

fn drawAlbedo() {}

fn drawNormals() {}
