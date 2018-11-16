extern crate image;
extern crate tobj;
extern crate nalgebra;

use image::{RgbImage, Rgb};
use std::path::Path;
use tobj::{Mesh,Model,Material};
use nalgebra::{Point, Point2, Point3, Vector3};

//probably gonna swap these out with nalgebra lib
#[derive(Debug, Clone, Copy)]
struct Vertex {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, Copy)]
struct FloatVertex {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Clone, Copy)]
struct BBox {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}
impl FloatVertex {
    fn from_vertex(v: Vertex) -> Self {
        FloatVertex{x: v.x as f32, y: v.y as f32, z: v.z as f32}
    }
}

pub trait Floatify3 {
    fn to_float(& self) -> Point3<f32>;
}

impl Floatify3 for Point3<i32> {
    fn to_float(& self) -> Point3<f32> {
        Point3::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

fn main() {
    let obj_file = tobj::load_obj(&Path::new("african_head.obj"));
    assert!(obj_file.is_ok());
    let (models, materials) = obj_file.unwrap();
    println!("# of models: {}", models.len());
    println!("# of materials: {}", materials.len());
    let scale = 511.0;
    let mut img = RgbImage::new((scale + 1.0) as u32,(scale + 1.0) as u32);
    let mesh = &models[0].mesh;
    let v = Point2::new(1,2);
    println!("{}", v.y);

    //idx is a list of indices for each face
//    for (i, m) in models.iter().enumerate() {
//        let mesh = &m.mesh;
//        assert!(mesh.positions.len() % 3 ==0);
//        println!("model[{}].name = \'{}\'", i, m.name);
//   wireframe(mesh, & mut img, Rgb([255,255,255]), scale);
//    }
    //triangle(Vertex{x:100, y:100, z:0}, Vertex{x:40, y:70, z:0}, Vertex{x:8, y:8, z:0}, & mut img, Rgb([255,255,255]));

    //img = image::imageops::flip_vertical(& img);
    //img.save("face.png").unwrap();
}
//This is a monstrosity... please kill it and remake. Remaking is harder than it looks.
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

fn horizontal_line(x0: i32,x1:i32, y:i32, img: & mut RgbImage,color: Rgb<u8>){
    for x in x0..=x1 {
        img.put_pixel(x as u32, y as u32, color);
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
//Go from bottom to top and make left and right side bounds.
//Everything passing image around, maybe make a class later.
fn triangle(v0: Point3<i32>, v1: Point3<i32>, v2: Point3<i32>, img: & mut RgbImage,color: Rgb<u8>){
    let mut v_high: Point3<i32> = v0;
    let mut v_mid: Point3<i32> = v1;
    let mut v_low: Point3<i32> = v2;
    //Sorting high to low
    if v_high.y < v_mid.y { std::mem::swap(& mut v_high, & mut v_mid)};
    if v_high.y < v_low.y { std::mem::swap(& mut v_high, & mut v_low)};
    if v_mid.y < v_low.y { std::mem::swap(& mut v_mid, & mut v_low)};

    let dx_mid_low = if v_mid.y > v_low.y {(v_mid.x  as f32 - v_low.x  as f32)/(v_mid.y  as f32 - v_low.y  as f32)} else {0.0};
    let dx_high_low = if v_high.y > v_low.y {(v_high.x  as f32 - v_low.x  as f32)/(v_high.y  as f32 - v_low.y  as f32)} else {0.0};
    let dx_high_mid = if v_high.y > v_mid.y {(v_high.x  as f32 - v_mid.x  as f32)/(v_high.y  as f32 - v_mid.y  as f32)} else {0.0};

    println!("mid_low: {}\nhigh_low: {}\nhigh_mid: {}", dx_mid_low, dx_high_low, dx_high_mid);
    let mut s: Point3<f32> = Point3::to_float(& v_low);
    let mut e: Point3<f32> = Point3::to_float(& v_low);
    if dx_mid_low > dx_high_low {
        while s.y < v_mid.y as f32 {
            horizontal_line(s.x.round() as i32, e.x.round() as i32, s.y as i32, img, color);
            e.y+=1.0;
            s.y+=1.0;
            s.x+=dx_high_low;
            e.x+=dx_mid_low;
        }
        e = Point3::to_float(& v_mid);
        while s.y <= v_high.y as f32 {
            horizontal_line(s.x.round() as i32, e.x.round() as i32, s.y as i32, img, color);
            s.y+=1.0;
            e.y+=1.0;
            s.x+=dx_high_low;
            e.x+=dx_high_mid;
        }
    } else {
        while s.y < v_mid.y as f32 {
            horizontal_line(s.x.round() as i32, e.x.round() as i32, s.y as i32, img, color);
            e.y+=1.0;
            s.y+=1.0;
            s.x+=dx_mid_low;
            e.x+=dx_high_low;
            println!("e.x: {}", e.x);
        }
        s = Point3::to_float(& v_mid);
        println!("2nd half\ne.x: {}", e.x);
        while s.y <= v_high.y as f32{
            horizontal_line(s.x.round() as i32, e.x.round() as i32, s.y as i32, img, color);
            s.y+=1.0;
            e.y+=1.0;
            s.x+=dx_high_mid;
            e.x+=dx_high_low;
            println!("e.x: {}", e.x);
        }
    }
}

//c highest a lowest? Keeping track of all this flipping is painful
fn barycentric_coords(a : Point3<i32>, b : Point3<i32>, c : Point3<i32>, p : Point3<i32>) -> Vector3<f32> {
    let x_component : Vector3<f32> = Vector3::new((c.x - a.x) as f32, (b.x - a.x) as f32, (a.x - p.x) as f32);
    let y_component : Vector3<f32> = Vector3::new((c.y - a.y) as f32, (b.y - a.y) as f32, (a.y - p.y) as f32);
    let u : Vector3<f32> = x_component.cross(& y_component);
    if u.z < 1.0 { //degenerate triangle
        return Vector3::new(-1.0, 1.0, 1.0)
    } else {
        Vector3::new(1.0-(u.x+u.y)/u.z, u.y/u.z, u.x/u.z)
    }
}

fn compute_bbox_triangle(v_high: Point3<i32>, v_mid: Point3<i32>, v_low: Point3<i32>) -> BBox {
    let y_max = v_high.y;//v_high is the highest y point v_low is lowest y
    let y_min = v_low.y;
    let mut x_min : i32 = v_low.x;
    let mut x_mid : i32 = v_mid.x;
    let mut x_max : i32 = v_high.x;
    if x_max < x_mid { std::mem::swap(& mut x_max, & mut x_mid)};
    if x_max < x_min { std::mem::swap(& mut x_max, & mut x_min)};
    if x_mid < x_min { std::mem::swap(& mut x_mid, & mut x_min)};
    BBox{min_x: x_min, min_y: y_min, max_x: x_max, max_y: y_max}
}