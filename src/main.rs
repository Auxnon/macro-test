use macroquad::prelude::*;

mod controls;
mod global;
mod image_helper;
mod menu;
mod shader;

use global::Global;

#[macroquad::main("Kiwi")]
async fn main() {
    let mut globals: Global = Default::default();

    let iwidth = (screen_width() as u16) / 4;
    let iheight = (screen_height() as u16) / 4;

    let birb: Texture2D = load_texture("assets/birbo.png").await.unwrap();
    let birb_n_img: Image = load_image("assets/birbo_n.png").await.unwrap();
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
        shader::SCREEN_VERTEX_SHADER,
        shader::SCREEN_FRAGMENT_SHADER,
        MaterialParams {
            uniforms: vec![
                ("Center".to_owned(), UniformType::Float2),
                ("ray".to_owned(), UniformType::Float2),
                ("resolution".to_owned(), UniformType::Float2),
                ("ratio".to_owned(), UniformType::Float1),
            ],
            textures: vec![
                //"Texture".to_owned() // this one is defined by Macroquad. assign other manually if needed.
                "normals".to_owned(),
                "albedo".to_owned(),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    let mut anim: u8 = 0;
    let max = (birb.width() / birb.height()) as u8;
    let mut tick: u8 = 0;
    let mut array: Vec<(f32, f32, bool)> = Vec::new();
    let mut array_is_dirty: bool = false;
    array.push((3., 128., true));

    let mut iter = 0;

    loop {
        let mw = screen_width() / 2.;
        let mh = screen_height() / 2.;
        let ir = screen_width() / 320.;
        let pixHeight = screen_height() / ir;

        tick += 1;
        if tick >= 6 {
            tick = 0;
            anim += 1;
            if anim >= max {
                anim = 0;
            }
        }

        let lens_center = mouse_position();

        clear_background(BLACK);
        //image.set_pixel(rand::gen_range(0, iwidth.into()), rand::gen_range(0, iheight.into()), RED);
        for i in 0..32 {
            for j in 0..32 {
                let n = 64; //(if lens_center.1 >iheight {iheight} else {lens_center.1}) as u16;
                image.get_image_data_mut()
                    [((i + n) * iwidth + j + lens_center.0 as u16) as usize] =
                    birbImg.get_image_data()[(i * 32 + j) as usize];
            }
        }

        let delta = (
            (lens_center.0 / screen_width()) - 0.5,
            (lens_center.1 / screen_height()) - 0.5,
        );
        let r = (delta.0 * delta.0 + delta.1 * delta.1).sqrt();

        screen_material.set_uniform("Center", lens_center);
        screen_material.set_uniform("ray", (delta.0 / r, delta.1 / r));
        screen_material.set_uniform("resolution", (320. as f32, pixHeight as f32));
        screen_material.set_uniform("ratio", ir);
        /* ========
        * normals
        =========*/
        for i in 0..array.len() {
            let dir = array[i].2;
            array[i].0 += if dir { 0.5 } else { -0.5 };
            let x = array[i].0;
            let y = array[i].1;

            if x > 320. {
                //x-=2.;
                array[i].2 = !dir;
            } else if x < 0. {
                //x+=2.;
                //
                //i-=1;
                //array[i].0=-99.; //we'll mark it dead on the x position like as a weird work around
                //array_is_dirty=true;

                array[i].2 = !dir;
            }
            draw_texture_ex(
                birb_n, //if dir {birb_n} else {birb_nf},
                x - 16.,
                y - 16. + 384.,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new((anim as f32) * 32., 0., 32., 32.)),
                    flip_x: dir,
                    ..Default::default()
                },
            );
        }
        set_default_camera();

        texture.update(&image);

        render_pass_first.update(&get_screen_data()); //dump our screen texture to our render_pass_first variable
        screen_material.set_texture("normals", render_pass_first); //send this screen capture to our shader

        clear_background(WHITE);
        //draw_texture(textureFirst, 64.,64.,WHITE);
        //get_active_render_pass();
        //texture()
        //draw_mode()

        /* ========
        * regular
        =========*/
        for i in 0..array.len() {
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
        }

        render_pass_second.update(&get_screen_data());
        screen_material.set_texture("albedo", render_pass_second); //send this screen capture to our shader

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
        clear_background(WHITE);
        gl_use_material(screen_material);
        draw_rectangle(0., 0., screen_width(), screen_height(), RED);
        //draw_circle(screen_width()/2., screen_height()/2., 350.0, RED);
        gl_use_default_material();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            iter += 32;
            if iter > 192 {
                iter = 0;
            }
            array.push((3., iter as f32, true)); //and::gen_range(20., 160.)
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

        next_frame().await
    }
}

fn drawAlbedo() {}

fn drawNormals() {}
