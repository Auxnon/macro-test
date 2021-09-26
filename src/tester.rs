fn init() {
    let test_texture = load_texture("assets/birb.png").await.unwrap();

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

    let mut meshes: Vec<macroquad::models::Mesh> = vec![];
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
            let verts_interleaved = izip!(
                reader.read_positions().unwrap(),
                //reader.read_normals().unwrap(),
                //reader.read_colors(0).unwrap().into_rgb_f32().into_iter(),
                reader.read_tex_coords(0).unwrap().into_f32(),
                //reader.read_indices().unwrap()
            );

            /*if let (Some(verts), Some(uvs)) = (
                reader.read_positions().map(|v| v),
                reader.read_tex_coords(0).map(|u| u),
            ) {
            }*/
            /*
            pub struct Mesh {
                pub vertices: Vec<Vertex>,
                pub indices: Vec<u16>,
                pub texture: Option<Texture2D>,
            }*/

            //);
            /* pub struct Vertex {
                pos: [f32; 3],
                uv: [f32; 2],
                color: [u8; 4],
            }*/
            pub struct Vertex {
                pub position: Vec3,
                pub uv: Vec2,
                pub color: Color,
            }

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
                    texture: Some(test_texture),
                };
                meshes.push(mesh);
            };

            //let inds=reader.read_indices().map(|i| i.into_u32().collect());
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
}

fn cycle() {
    if false {
        set_camera(&Camera3D {
            //position: vec3(0.001, 1., 0.),
            position: vec3(1., (incr_time * 20.) as f32, 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });
    }
    draw_cube(
        Vec3::new(0., 0., 0.),
        Vec3::new(10., 2., 10.),
        render_pass_first,
        WHITE,
    );
    match meshes.get(0) {
        Some(o) => {
            draw_mesh(o);
        }
        None => {
            println!("naw")
        }
    }
}

fn graveyard() {
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

    //let birb: Texture2D = load_texture("assets/birb.png").await.unwrap();
    //let birb_n_img: Image = load_image("assets/birb_n.png").await.unwrap();
    //let birb_n: Texture2D = Texture2D::from_image(&birb_n_img); //load_texture("assets/birbo_n.png").await.unwrap();
    //let immm = image_helper::flip(&birb_n_img, 0);
    //let birb_nf: Texture2D = Texture2D::from_image(&immm);

    //let mut image = Image::gen_image_color(iwidth, iheight, WHITE);
    //let texture = Texture2D::from_image(&image);
    //texture.set_filter(FilterMode::Nearest);

    //let birbImg: Image = load_image("assets/kiwi.png").await.unwrap();
    //birb.set_filter(FilterMode::Nearest);
    //let chess: Texture2D = load_texture("assets/chess.png").await.unwrap();
    //chess.set_filter(FilterMode::Nearest);

    //let mut array: Vec<(f32, f32, bool)> = Vec::new();
    //let mut array_is_dirty: bool = false;
    //array.push((3., 128., true));

    //let mut iter: u32 = 0;

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
}
