use macroquad::prelude::*;

pub fn init() {}
pub fn render(tick: f32, texture: Texture2D) {
    set_camera(&Camera3D {
        position: vec3(-20. + (tick) * 40., 15., 0.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        viewport: Some((0, (screen_height() - 192.) as i32, 320, 192)),
        ..Default::default()
    });

    let size = Vec3::new(10., 10., 10.);
    draw_cube(Vec3::new(0., 0., 0.), size, texture, RED);

    draw_grid(20, 1., BLACK, GRAY);

    draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
    draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
    draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), YELLOW);

    draw_plane(vec3(-8., 0., -8.), vec2(5., 5.), texture, WHITE);

    draw_cube(vec3(-5., 1., -2.), vec3(2., 2., 2.), texture, WHITE);
    draw_cube(vec3(-5., 1., 2.), vec3(2., 2., 2.), texture, WHITE);
    draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);

    draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);

    // Back to screen space, render some text
    set_camera(&Camera2D {
        ..Default::default()
    });

    set_default_camera();
}
