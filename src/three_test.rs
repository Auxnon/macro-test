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

pub fn render(
    delta: f32,

    incr_time: f32,
    texture: Texture2D,
    texture2: Texture2D,
    ents: &mut Vec<Ent>,
    tick: bool,
) {
    // set_camera(&Camera3D {
    //     position: vec3(-20. + (tick) * 40., 15., 0.),
    //     up: vec3(0., 1., 0.),
    //     target: vec3(0., 0., 0.),
    //     viewport: Some((0, (screen_height() - 192.) as i32, 320, 192)),
    //     ..Default::default()
    // });
    let r = (incr_time * 1.) % std::f32::consts::PI * 2.;
    let d = 3.;
    let x = r.cos() * d;
    let y = r.sin() * d;
    let camera = &Camera3D {
        position: vec3(x, d, y),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        viewport: Some((0, (screen_height() - 192.) as i32, 320, 192)),
        ..Default::default()
    };

    let matrix = camera.matrix().clone();
    set_camera(camera);

    let size = Vec3::new(10., 10., 10.);
    //draw_cube(Vec3::new(0., 0., 0.), size, texture, RED);

    draw_grid(20, 1., BLACK, GRAY);

    draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
    draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
    //draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2.1, 2.), YELLOW);

    //draw_plane(vec3(0., 0.01, 0.), vec2(2., 2.), texture, WHITE);
    if true {
        draw_plane(vec3(-2., 0., -2.), vec2(2., 2.), texture, WHITE);
        draw_plane(vec3(-2., 0., 2.), vec2(2., 2.), texture, WHITE);
        draw_plane(vec3(2., 0., 2.), vec2(2., 2.), texture, WHITE);
        draw_plane(vec3(2., 0., -2.), vec2(2., 2.), texture, WHITE);
        draw_cube(vec3(-5., 1., -2.), vec3(2., 2., 2.), texture, WHITE);
    }
    let gl = unsafe { get_internal_gl() };

    let (scale, quat, trans) = matrix.inverse().to_scale_rotation_translation();
    let mut standing_matrix = Mat4::from_axis_angle(vec3(1., 0., 0.), std::f32::consts::PI * 0.5);

    for ent in ents {
        ent.run(delta);
        if ent.is_flat() {
            let rot_matrix = Mat4::from_rotation_translation(quat, ent.pos);
            gl.quad_gl
                .push_model_matrix(rot_matrix.mul_mat4(&standing_matrix));
        } else {
            gl.quad_gl.push_model_matrix(ent.matrix);
        }
        ent.draw(delta, tick, false, true);

        gl.quad_gl.pop_model_matrix();
    }

    //gl.quad_gl.pop_model_matrix();

    // Back to screen space, render some text
    set_camera(&Camera2D {
        ..Default::default()
    });

    set_default_camera();
}
