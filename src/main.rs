
use macroquad::prelude::*;
use std::num;


mod imagehelper;


#[macroquad::main("Kiwi")]
async fn main() {

    let iwidth=(screen_width()as u16)/4;
    let iheight=(screen_height()as u16)/4;

    let birb: Texture2D = load_texture("assets/birbo.png").await.unwrap();
    let birb_n_img: Image = load_image("assets/birbo_n.png").await.unwrap();
    let birb_n: Texture2D =Texture2D::from_image(&birb_n_img); //load_texture("assets/birbo_n.png").await.unwrap();
    let immm=imagehelper::flip_red(&birb_n_img);
    println!("returned ${}", immm.get_image_data()[((13) as usize)][2]);
    let birb_nf: Texture2D = Texture2D::from_image(&immm);

    let mut image = Image::gen_image_color(iwidth,iheight, WHITE);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    
    
   
    
    let im=get_screen_data();
    let textureFirst = Texture2D::from_image(&im);
    textureFirst.set_filter(FilterMode::Nearest);

    let stage = {
        let InternalGlContext {
            quad_context: ctx, ..
        } = unsafe { get_internal_gl() };

        raw_miniquad::Stage::new(ctx,texture.raw_miniquad_texture_handle()) //
    };
    
    //let r=macroquad::texture::render_target(64, 64);
    //macroquad::camera::Camera2D::default().render_target=Some(r);
    //macroquad::camera::Camera2D::from_display_rect(Rect::new(0.,0.,320.,180.));
    let birbImg:Image =load_image("assets/kiwi.png").await.unwrap();
    
    birb.set_filter(FilterMode::Nearest);
    let chess: Texture2D = load_texture("assets/chess.png").await.unwrap();
    chess.set_filter(FilterMode::Nearest);



    let middle=(screen_width()/2.,screen_width()/2.);

    

    let lens_material = load_material(
        LENS_VERTEX_SHADER,
        LENS_FRAGMENT_SHADER,
        MaterialParams {
            uniforms: vec![("Center".to_owned(), UniformType::Float2),("ray".to_owned(), UniformType::Float2),("resolution".to_owned(), UniformType::Float2)],
            textures: vec![
                //"Texture".to_owned() // this one is defined by Macroquad. assign other manually if needed.
                "normals".to_owned()
                ],
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

        let lens_center = mouse_position();
        
        clear_background(WHITE);
        /*draw_texture_ex(
            texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );*/
       
        

        //image.set_pixel(rand::gen_range(0, iwidth.into()), rand::gen_range(0, iheight.into()), RED);
        for i in 0..32{
            for j in 0..32{
                let n=64;//(if lens_center.1 >iheight {iheight} else {lens_center.1}) as u16;
                image.get_image_data_mut()[((i+n)*iwidth + j+lens_center.0 as u16) as usize]=birbImg.get_image_data()[(i*32+j) as usize];
            }
        }
        
        //draw_texture(texture,0.,0.,WHITE);
       
        let delta=((lens_center.0/screen_width())-0.5,(lens_center.1/screen_height())-0.5);
        let r=(delta.0*delta.0 + delta.1*delta.1).sqrt();

        lens_material.set_uniform("Center", lens_center);
        lens_material.set_uniform("ray", (delta.0/r,delta.1/r));
        lens_material.set_uniform("resolution", (iwidth as f32,iheight as f32));
        

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
                //array[i].0=-99.; //we'll mark it dead on the x position like as a weird work around
                //array_is_dirty=true;
                
                array[i].2=!dir;
            }
            
            draw_texture_ex(
                if dir {birb_n} else {birb_nf},
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
        


/*
        {
            let mut gl = unsafe { get_internal_gl() };

            // Ensure that macroquad's shapes are not going to be lost
            gl.flush();

            let t = get_time();
           // gl.quad_context.

            gl.quad_context.apply_pipeline(&stage.pipeline);

            gl.quad_context
                .begin_default_pass(miniquad::PassAction::Nothing);
            gl.quad_context.apply_bindings(&stage.bindings);

            //for i in 0..3 {
                let t = t + 0 as f64 * 0.3;

                gl.quad_context
                    .apply_uniforms(&raw_miniquad::shader::Uniforms {
                        offset: (t.sin() as f32 * 0.5, (t * 3.).cos() as f32 * 0.5),
                    });
                gl.quad_context.draw(0, 6, 1);
            //}
            gl.quad_context.end_render_pass();
        }*/

        // Back to screen space, render some text

        set_default_camera();

       
        texture.update(&image);

        textureFirst.update(&get_screen_data());
        lens_material.set_texture("normals", textureFirst);

        clear_background(WHITE);
        //draw_texture(textureFirst, 64.,64.,WHITE);
        //get_active_render_pass();
        //texture()
        //draw_mode()



        for i in 0..array.len() {
            //let dir=array[i].2;
            draw_texture_ex(
                birb,
                array[i].0-16.,
                array[i].1-16.,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new((anim as f32)*32.,0.,32.,32.)),
                    dest_size: Some(vec2(128.,128.)),
                    flip_x:array[i].2,
                    ..Default::default()
                },
            );
        }
       
        gl_use_material(lens_material);
        draw_rectangle( 0.,0.,screen_width(),screen_height(), RED);
        //draw_circle(screen_width()/2., screen_height()/2., 350.0, RED);
        
        gl_use_default_material();
            
        if is_key_pressed(KeyCode::Escape){
            break;
        }
        if is_key_pressed(KeyCode::Space){
            array.push((3.,rand::gen_range(64., 256.),true));
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
        
        next_frame().await
    }
}

const LENS_FRAGMENT_SHADER: &'static str = r#"#version 100
precision lowp float;
varying vec2 uv;
varying vec2 uv_screen;
varying vec2 center;

uniform vec2 ray;
uniform vec2 resolution;
uniform sampler2D normals;
uniform sampler2D _ScreenTexture;
void main() {
    float gradient = length(uv);
    vec2 vector=(uv_screen - center);
    vec2 uv_zoom = vector * gradient + center;
    vec2 n=normalize(center);

    vec4 col = texture2D(_ScreenTexture, uv_screen);
    if(col.a>.1){
        vec2 ints=vec2(floor(uv_screen.x*resolution.x)/resolution.x,floor(uv_screen.y*resolution.y)/resolution.y);
        vec4 norms=texture2D(normals,ints);
        if(norms.b<1.){
            vec3 n=normalize(vec3(norms.r-.5,.5-norms.g,norms.b-.5));
            vec2 v=normalize(vector);
            float c=normalize(vec2(n.x*v.x,n.y*v.x)).x;
            //t = glm::normalize(t - n * glm::dot(n, t));
            //vec2 uv2 = normalize(ray-n*dot(n,ray));//(uv-0.5*uv_screen.xy)/uv_screen.y;
            float f = dot(vec3(ray,0),n);
           // float f2 = dot(vec3((ray+vec2(.1,0),0)),n);
            //float f3 = dot(vec3((ray-vec2(.1,0),0)),n);


            //f=floor(f*3.)/3.;
/*
            if(floor(mod((ints.x*resolution.x+ints.y*resolution.y),(1.-f)*3.))==0.){
                f=1.;
            } else{
                f=0.;
            }*/
        
            //col=vec4(1,0,1,0);
            gl_FragColor = mix(col*.6,col,f);//vec4(f,0.,0.,f);
        }else{
            gl_FragColor = vec4(col);
        }
    }else{ 
        gl_FragColor = vec4(col);
    }
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
uniform vec2 ray;
void main() {
    vec4 res = Projection * Model * vec4(position, 1);
    vec4 c = Projection * Model * vec4(Center, 0, 1);
    uv_screen = res.xy / 2.0 + vec2(0.5, 0.5);
    
    center = c.xy / 2.0 + vec2(0.5, 0.5);
    uv = texcoord;
    gl_Position = res;
}
";





mod raw_miniquad {
    use miniquad::*;

    #[repr(C)]
    struct Vec2 {
        x: f32,
        y: f32,
    }
    #[repr(C)]
    struct Vertex {
        pos: Vec2,
        uv: Vec2,
    }

    pub struct Stage {
        pub pipeline: Pipeline,
        pub bindings: Bindings,
    }

    impl Stage {
        pub fn new(ctx: &mut Context,texture1:Texture) -> Stage { //texture: macroquad::texture::Texture2D
            #[rustfmt::skip]
            let vertices: [Vertex; 4] = [
                Vertex { pos : Vec2 { x: -0.5, y: -0.5 }, uv: Vec2 { x: 0., y: 0. } },
                Vertex { pos : Vec2 { x:  0.5, y: -0.5 }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x:  0.5, y:  0.5 }, uv: Vec2 { x: 1., y: 1. } },
                Vertex { pos : Vec2 { x: -0.5, y:  0.5 }, uv: Vec2 { x: 0., y: 1. } },
            ];
            let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

            let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
            let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

            let pixels: [u8; 4 * 4 * 4] = [
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
                0x00, 0xFF, 0x45, 0xa0, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            ];
            let texture = Texture::from_rgba8(ctx, 4, 4, &pixels);

            let bindings = Bindings {
                vertex_buffers: vec![vertex_buffer],
                index_buffer,
                images: vec![ texture],
            };

            let shader =
                Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

            let pipeline = Pipeline::new(
                ctx,
                &[BufferLayout::default()],
                &[
                    VertexAttribute::new("pos", VertexFormat::Float2),
                    VertexAttribute::new("uv", VertexFormat::Float2),
                ],
                shader,
            );

            Stage { pipeline, bindings }
        }
    }

    pub mod shader {
        use miniquad::*;

        pub const VERTEX: &str = r#"#version 100
attribute vec2 pos;
attribute vec2 uv;

uniform vec2 offset;

varying lowp vec2 texcoord;

void main() {
    gl_Position = vec4(pos + offset, 0, 1);
    texcoord = uv;
}"#;

        pub const FRAGMENT: &str = r#"#version 100
varying lowp vec2 texcoord;

uniform sampler2D tex;

void main() {
    gl_FragColor = texture2D(tex, texcoord);
}"#;

        pub fn meta() -> ShaderMeta {
            ShaderMeta {
                images: vec!["tex".to_string()],
                uniforms: UniformBlockLayout {
                    uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
                },
            }
        }

        #[repr(C)]
        pub struct Uniforms {
            pub offset: (f32, f32),
        }
    }
}
