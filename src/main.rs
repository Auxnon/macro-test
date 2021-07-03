
use macroquad::prelude::*;


#[macroquad::main("Kiwi")]
async fn main() {
    //let r=macroquad::texture::render_target(64, 64);
    //macroquad::camera::Camera2D::default().render_target=Some(r);
    //macroquad::camera::Camera2D::from_display_rect(Rect::new(0.,0.,320.,180.));

    let birb: Texture2D = load_texture("assets/birbo.png").await.unwrap();
    birb.set_filter(FilterMode::Nearest);
    let chess: Texture2D = load_texture("assets/chess.png").await.unwrap();
    chess.set_filter(FilterMode::Nearest);

    let iwidth=(screen_width()as u16)/4;
    let iheight=(screen_height()as u16)/4;

    let mut image = Image::gen_image_color(iwidth,iheight, WHITE);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let lens_material = load_material(
        LENS_VERTEX_SHADER,
        LENS_FRAGMENT_SHADER,
        MaterialParams {
            uniforms: vec![("Center".to_owned(), UniformType::Float2)],
            ..Default::default()
        },
    )
    .unwrap();

    



    let mut anim:u8=0;
    let max=(birb.width()/birb.height())as u8;
    let mut tick:u8=0;
    let mut array:Vec<(f32,f32,bool)> = Vec::new();
    let mut array_is_dirty:bool=false;
    array.push((3.,128.,true));

    loop {
        tick+=1;
        if tick>=6{
            tick=0;
            anim+=1;
            if anim>=max{
                anim=0;
            }
        }
        
            
        clear_background(WHITE);
        draw_texture_ex(
            texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        image.set_pixel(rand::gen_range(0, iwidth.into()), rand::gen_range(0, iheight.into()), RED);
        texture.update(&image);
        //draw_texture(texture,0.,0.,WHITE);

        let lens_center = mouse_position();

        lens_material.set_uniform("Center", lens_center);

        gl_use_material(lens_material);
        draw_circle(lens_center.0, lens_center.1, 250.0, RED);
        gl_use_default_material();


        for i in 0..array.len() {
            let dir=array[i].2;
            array[i].0+=if dir {4.} else {-4.};
            let x=array[i].0;
            let y=array[i].1;
            
            if x>screen_width(){
                //x-=2.;
                array[i].2=!dir;
            }else if x<0.{
                //x+=2.;
                //
                //i-=1;
                array[i].0=-99.; //we'll mark it dead on the x position like as a weird work around
                array_is_dirty=true;
                
                //array[i].2=!dir;
            }
            
            draw_texture_ex(
                birb,
                x-16.,
                y-16.,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new((anim as f32)*32.,0.,32.,32.)),
                    dest_size: Some(vec2(128.,128.)),
                    flip_x:dir,
                    ..Default::default()
                },
            );
        }
        //dump our dead bois safely outside the entity loop array
        if array_is_dirty{
            //info!("dirty");
            for i in (0..array.len()).rev() {
                if array[i].0==-99.{
                    //info!("dumped and now array now size {}",array.len());
                    array.remove(i);
                }
            }
            
            array_is_dirty=false;
        }
        
       

        
        
        if is_key_pressed(KeyCode::Escape){
            break;
        }
        if is_key_pressed(KeyCode::Space){
            array.push((3.,rand::gen_range(64., 256.),true));
        }
        

        next_frame().await
    }
}

const LENS_FRAGMENT_SHADER: &'static str = r#"#version 100
precision lowp float;
varying vec2 uv;
varying vec2 uv_screen;
varying vec2 center;
uniform sampler2D _ScreenTexture;
void main() {
    float gradient = length(uv);
    vec2 uv_zoom = (uv_screen - center) * gradient + center;
    gl_FragColor = texture2D(_ScreenTexture, uv_zoom);
}
"#;

const LENS_VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying lowp vec2 center;
varying lowp vec2 uv;
varying lowp vec2 uv_screen;
uniform mat4 Model;
uniform mat4 Projection;
uniform vec2 Center;
void main() {
    vec4 res = Projection * Model * vec4(position, 1);
    vec4 c = Projection * Model * vec4(Center, 0, 1);
    uv_screen = res.xy / 2.0 + vec2(0.5, 0.5);
    center = c.xy / 2.0 + vec2(0.5, 0.5);
    uv = texcoord;
    gl_Position = res;
}
";