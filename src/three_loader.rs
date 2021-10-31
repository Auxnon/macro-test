use gltf::{image::Data as ImageData, json::extensions::mesh, Texture};
use itertools::izip;
use macroquad::prelude::*;

pub fn load(str: &str) -> Vec<Mesh> {
    let (nodes, buffers, image_data) = gltf::import(str).unwrap();
    let mut meshes: Vec<Mesh> = vec![];
    let im1 = image_data.get(0).unwrap();
    let tex = Texture2D::from_rgba8(im1.width as u16, im1.height as u16, &im1.pixels);
    tex.set_filter(FilterMode::Nearest);
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
                rand::srand(6);

                //mat.mul_vec4(other)
                meshes.push(mesh);
            };
        }
    }
    return meshes;
}
