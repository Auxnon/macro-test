use macroquad::prelude::*;

use crate::ent::Ent;

pub struct Ent3 {
    mesh: macroquad::models::Mesh,
    mat: Mat4,
}
// pub fn init(texture: Texture2D, ent_factory: EntFactory) -> Vec<Mesh> {
//     let mut meshes: Vec<Mesh> = vec![];
//     //meshes.append(&mut crate::three_loader::load("assets/lil-house.glb"));
//     //meshes.append(&mut load("assets/console.glb"));
//     //ent_factory.re
//     //meshes
// }

pub fn render(tick: f32, texture: Texture2D, texture2: Texture2D, ents: &mut Vec<Ent>) {
    // set_camera(&Camera3D {
    //     position: vec3(-20. + (tick) * 40., 15., 0.),
    //     up: vec3(0., 1., 0.),
    //     target: vec3(0., 0., 0.),
    //     viewport: Some((0, (screen_height() - 192.) as i32, 320, 192)),
    //     ..Default::default()
    // });
    let r = (tick * 2.) % std::f32::consts::PI;
    let d = 3.;
    let x = r.cos() * d;
    let y = r.sin() * d;
    set_camera(&Camera3D {
        position: vec3(x, d, y),
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
    //draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2.1, 2.), YELLOW);

    unsafe {
        macroquad::window::get_internal_gl()
            .quad_gl
            .pop_model_matrix();
    }
    draw_plane(vec3(0., 0.01, 0.), vec2(2., 2.), texture, WHITE);
    draw_cube(vec3(-5., 1., -2.), vec3(2., 2., 2.), texture, WHITE);
    draw_cube(vec3(-5., 1., 2.), vec3(2., 2., 2.), texture, WHITE);
    draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);
    //draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);
    let gl = unsafe { get_internal_gl() };
    for ent in ents {
        ent.run(0.1);
        let mat2 = Mat4::look_at_lh(vec3(0., 0., 1.), ent.pos, vec3(0., 1., 0.));
        //mat2.mul_mat4(&ent.matrix);
        gl.quad_gl.push_model_matrix(mat2);
        for m in &ent.get_schema().mesh {
            draw_mesh(m);
        }
        gl.quad_gl.pop_model_matrix();
    }

    let mat = glam::Mat4::from_axis_angle(Vec3::new(1., 0., 0.), std::f32::consts::PI * 0.5);
    gl.quad_gl.push_model_matrix(mat);

    draw_plane(
        vec3(1. + 1. / 4., -1.25, -1. / 4.),
        vec2(1. / 4., 1. / 4.),
        texture2,
        WHITE,
    );
    gl.quad_gl.pop_model_matrix();

    // Back to screen space, render some text
    set_camera(&Camera2D {
        ..Default::default()
    });

    set_default_camera();
}

fn model() {}
