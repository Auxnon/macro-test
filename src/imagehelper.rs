use macroquad::texture::Image;

pub fn flip_red(img: &Image) -> Image {
    let w=img.width() as u32;
    let mut im = img.clone();
    //println!("got ${}", im.bytes[13]);//im.get_image_data()[((13) as usize)][2]);
    for i in 0..w {
        for j in 0..32 {
            //let n=64;//(if lens_center.1 >iheight {iheight} else {lens_center.1}) as u16;
            // let c=img.get_image_data()[(i*32+j) as usize][0];
            //print!("| {}",im.get_image_data()[(i * 32 as u32 + j) as usize][0]);
            //print!(",{}",im.get_image_data()[(i * 32 as u32 + j) as usize][1]);
            //print!(",{}",im.get_image_data()[(i * 32 as u32 + j) as usize][2]);
            //print!(",{} |",im.get_image_data()[(i * 32 as u32 + j) as usize][3]);

           /* im.get_image_data_mut()[(i * 32 as u32 + j) as usize][0]=0.into();
            im.get_image_data_mut()[(i * 32 as u32 + j) as usize][1]=0.into();
            im.get_image_data_mut()[(i * 32 as u32 + j) as usize][2]=0.into();
            im.get_image_data_mut()[(i * 32 as u32 + j) as usize][3]=0.into();*/

            //let v= macroquad::prelude::RED;//[1, 2, 3,1];
            let c=im.get_image_data()[(i * 32 + j) as usize][0];
            im.get_image_data_mut()[(i * 32 + j) as usize][0]=255-c;//v.into();
            //im.get_image_data_mut()[((i * 32 + j) as usize)].r=0.;

            //im.get_image_data_mut()[((i * 32 + j) as usize)][1]=0;
            //im.get_image_data_mut()[((i * 32 + j) as usize)][2]=0;
            //im.get_image_data_mut()[((i * 32 + j) as usize)][3]=0;

            //im.set_pixel(i,j,macroquad::prelude::RED);
            //im.bytes[i*32+j]=0.into();
            //Some(col)[0] = 0;
            //*col[2] = 0;
            //col[3] = 0;
            //=[0,c[1],c[2],c[3]];
            //print!("{}",c);
            //print!("got ${}",im.get_image_data_mut()[((i*32 + j) as usize)][0]);
        }
    }

    //println!("got ${}", im.get_image_data()[((13) as usize)][2]);

    im
}
