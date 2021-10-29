use gltf::{Texture,image::Data as ImageData};
use macroquad::prelude::*;
use itertools::izip;

pub fn init(texture:Texture2D)-> Vec<macroquad::models::Mesh>{
    let (nodes, buffers, image_data) = gltf::import("assets/console.gltf").unwrap();
    let mut meshes: Vec<macroquad::models::Mesh> = vec![];
    let im1=image_data.get(0).unwrap();
    let tex=Texture2D::from_rgba8(im1.width as u16, im1.height as u16, &im1.pixels);

    for mesh in nodes.meshes() {
  
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            let verts_interleaved = izip!(
                reader.read_positions().unwrap(),
                //reader.read_normals().unwrap(),
                //reader.read_colors(0).unwrap().into_rgb_f32().into_iter(),
                reader.read_tex_coords(0).unwrap().into_f32(),
                //reader.read_indices().unwrap()
            );

            let vertices = verts_interleaved
                .map(|(pos, uv)| macroquad::models::Vertex {
                    position: Vec3::from(pos),
                    uv: Vec2::from(uv),
                    color: WHITE,
                })
                .collect::<Vec<macroquad::models::Vertex>>();

            if let Some(inds) = reader.read_indices() {
                let indices = inds.into_u32().map(|u| u as u16).collect::<Vec<u16>>();
                let mesh = macroquad::models::Mesh {
                    vertices,
                    indices,
                    texture: Some(tex),
                };
                
                meshes.push(mesh);
            };
        }
    }
    return meshes;

}
pub fn render(tick: f32, texture: Texture2D,meshes:&Vec<macroquad::models::Mesh>) {
    // set_camera(&Camera3D {
    //     position: vec3(-20. + (tick) * 40., 15., 0.),
    //     up: vec3(0., 1., 0.),
    //     target: vec3(0., 0., 0.),
    //     viewport: Some((0, (screen_height() - 192.) as i32, 320, 192)),
    //     ..Default::default()
    // });
        let r=(tick*2.)%std::f32::consts::PI;
        let d=5.;
    let x=r.cos()*d;
    let y=r.sin()*d;
    set_camera(&Camera3D {
        position: vec3(x,5.,y),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        viewport: Some((0, (screen_height() - 192.) as i32, 320, 192)),
        ..Default::default()
    });

    let size = Vec3::new(10., 10., 10.);
    //draw_cube(Vec3::new(0., 0., 0.), size, texture, RED);
    
    draw_grid(20, 1., BLACK, GRAY);

    draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
    draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
    draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2.1, 2.), YELLOW);

    draw_plane(vec3(-8., 0., -8.), vec2(5., 5.), texture, WHITE);

    draw_cube(vec3(-5., 1., -2.), vec3(2., 2., 2.), texture, WHITE);
    draw_cube(vec3(-5., 1., 2.), vec3(2., 2., 2.), texture, WHITE);
    draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);
    //draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);

    let mat4=glam::Mat4::from_axis_angle(Vec3::new(0., 1., 0.), (24.*tick)%std::f32::consts::PI*2.);

    unsafe {
        macroquad::window::get_internal_gl().quad_gl.push_model_matrix(mat4);
    }
    

    for mesh in meshes{
        
        draw_mesh(&mesh);
    }
    
    unsafe{
        macroquad::window::get_internal_gl().quad_gl.pop_model_matrix();
    }
    // Back to screen space, render some text
    set_camera(&Camera2D {
        ..Default::default()
    });

    set_default_camera();
}


fn model(){

}
