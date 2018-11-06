extern crate image;
extern crate tobj;

use image::{RgbImage, Rgb};
use std::path::Path;
use tobj::{Mesh,Model,Material};

fn main() {
    let obj_file = tobj::load_obj(&Path::new("cb_ex003_a.obj"));
    assert!(obj_file.is_ok());
    let (models, materials) = obj_file.unwrap();
    println!("# of models: {}", models.len());
    println!("# of materials: {}", materials.len());
    let scale = 2048.0;
    let mut img = RgbImage::new((scale + 1.0) as u32,(scale + 1.0) as u32);

//    for i in 0..mesh.positions.len() / 3 {
//        println!("line: {}\nx: {},y: {},z: {}",i,mesh.positions[3*i], mesh.positions[3*i+1],mesh.positions[3*i+2])
//    }
    //idx is a list of indices for each face
    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        assert!(mesh.positions.len() % 3 ==0);
        println!("model[{}].name = \'{}\'", i, m.name);
        //wireframe(mesh, & mut img, Rgb([255,255,255]), scale);
    }

    // Normals and texture coordinates are also loaded, but not printed in this example
    //println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
    //assert!(mesh.positions.len() % 3 == 0);
    //for v in 0..mesh.positions.len() / 3 {
    //        println!("    v[{}] = ({}, {}, {})", v, mesh.positions[3 * v],
    //            mesh.positions[3 * v + 1], mesh.positions[3 * v + 2]);
    //}
    //}

//    img = image::imageops::flip_vertical(& img);
//    img.save("body.png").unwrap();
}
//This is a monstrosity... please kill it and remake.
fn line(x0: u32, y0:u32,x1:u32,y1:u32,img: & mut RgbImage,color: Rgb<u8>){
    let mut steep: bool = false;
    let mut mx0 = x0;
    let mut my0 = y0;
    let mut mx1 = x1;
    let mut my1 = y1;
    if (x0 as i32 - x1 as i32).abs()<(y0 as i32 - y1 as i32).abs() {//transposing
        mx0 = y0;
        my0 = x0;
        mx1 = y1;
        my1 = x1;
        steep = true;
    }
    if mx0 > mx1 {//always aiming left-right
        std::mem::swap(& mut mx0,& mut  mx1);
        std::mem::swap(& mut my0,& mut  my1);
    }
    let dx = (mx1 as i32)-(mx0 as i32);
    let dy = (my1 as i32)-(my0 as i32);
    let derror2 = dy.abs() * 2 as i32;
    let mut error2: i32 = 0;
    let mut y = my0;
    let mut x = mx0;
    while x<=mx1 {
        if steep {
            img.put_pixel(y, x, color); //detranspose transposed version
        } else {
            img.put_pixel(x, y, color);
        }
        error2+=derror2;
        if error2 > dx {//adding pixels when necessary
            if my1>my0 {y+=1} else {y-=1};
            error2 -= dx* 2;
        }       
        x+=1;
    }
}

fn iline(x0: i32, y0: i32, x1: i32, y1: i32,img: & mut RgbImage,color: Rgb<u8>) {
    if (x0 - x1).abs() < (y0 - y1).abs() {
        let range = if x1 < x0 { x1..x0 } else { x0..x1 };
        for x in range {
            let t = (x - x0) as f32 / (x1 - x0) as f32;
            let y = (y0 as f32 * (1.0 - t) + (y1 as f32 * t)) as i32;
            img.put_pixel(x as u32, y as u32, color);
        }
    } else {
        let range = if y1 > y0 { y0..y1 } else { y1..y0 };
        for y in range {
            let t = (y - y0) as f32 / (y1 - y0) as f32;
            let x = (x0 as f32 * (1.0 - t) + (x1 as f32 * t)) as i32;
            img.put_pixel(x as u32, y as u32, color);
        }
    }
}

fn wireframe(mesh: & Mesh, img: & mut RgbImage, color: Rgb<u8>, scale: f32){
    for f in 0..mesh.indices.len() / 3 {
        let x0 = ((mesh.positions[3 * (mesh.indices[3 * f])as usize]+1.0)*scale/2.0) as u32;//x
        let x1 = ((mesh.positions[3 * mesh.indices[3 * f + 1] as usize]+1.0)*scale/2.0) as u32;
        let x2 = ((mesh.positions[3 * mesh.indices[3 * f + 2] as usize]+1.0)*scale/2.0) as u32;
        let y0 = ((mesh.positions[3 * mesh.indices[3 * f] as usize +1]+1.0)*scale/2.0) as u32;
        let y1 = ((mesh.positions[3 * mesh.indices[3 * f + 1]as usize + 1]+1.0)*scale/2.0) as u32;
        let y2 = ((mesh.positions[3 * mesh.indices[3 * f + 2]as usize + 1]+1.0)*scale/2.0) as u32;
        line(x0,y0,x1,y1,img, color);
        line(x1,y1,x2,y2,img, color);
        line(x2,y2,x0,y0,img, color);
    }
}