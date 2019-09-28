extern crate image;
extern crate tobj;
extern crate nalgebra;
extern crate rand;

use image::{RgbImage, Rgb};
use std::path::Path;
use tobj::{Mesh,Model,Material};
use nalgebra::{Point, Point2, Point3, Vector3};
use rand::Rng;

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

enum Direction {
    X = 0,
    Y = 1,
    Z = 2,
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

    //idx is a list of indices for each face
//    for (i, m) in models.iter().enumerate() {
//        let mesh = &m.mesh;
//        assert!(mesh.positions.len() % 3 ==0);
//        println!("model[{}].name = \'{}\'", i, m.name);
    basic_render(mesh, & mut img, Rgb([255,255,255]), scale);
//    }
//   triangle(Point3::new(100, 100, 0), Point3::new(40, 70, 0), Point3::new(8, 8, 0), & mut img, Rgb([255,255,255]));

    img = image::imageops::flip_vertical(& img);
    img.save("basictest.png").unwrap();
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
    let get_coord = | index: usize, dir: usize, f: usize | ((mesh.positions[3 * (mesh.indices[3 * f + index])as usize + dir]+1.0)*scale/2.0) as u32;
    for f in 0..mesh.indices.len() / 3 {
        let x0 = get_coord(0,Direction::X as usize, f);
        let x1 = get_coord(1,Direction::X as usize, f);
        let x2 = get_coord(2,Direction::X as usize, f);
        let y0 = get_coord(0,Direction::Y as usize, f);
        let y1 = get_coord(1,Direction::Y as usize, f);
        let y2 = get_coord(2,Direction::Y as usize, f);
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
    let mut hit_flag: bool = false;
    if v_high.y < v_mid.y { std::mem::swap(& mut v_high, & mut v_mid)};
    if v_high.y < v_low.y { std::mem::swap(& mut v_high, & mut v_low)};
    if v_mid.y < v_low.y { std::mem::swap(& mut v_mid, & mut v_low)};

    let bbox : BBox = compute_bbox_triangle(v_high, v_mid, v_low);
    let mut barycentric : Vector3<f32>;
    let mut p : Point3<i32> = Point3::new(0, 0, 0);
    for x in bbox.min_x..=bbox.max_x {
        for y in bbox.min_y..=bbox.max_y {
            p.x = x;
            p.y = y;
            barycentric = barycentric_coords(v_low, v_mid, v_high, p);
            if (barycentric.x < 0.0) || (barycentric.y < 0.0) || (barycentric.z < 0.0) {
                //println!("Barycentric coords: {}, {}, {}",barycentric.x,barycentric.y,barycentric.z);
                continue;
            }
            hit_flag = true;
            img.put_pixel(x as u32, y as u32, color);
        }
    }
    if !hit_flag {
        println!("Triangle {}, {}, {} missed\nBBox min: {}, {}\nBBox max: {}, {}", v_high,v_mid,v_low,bbox.min_x, bbox.min_y, bbox.max_x, bbox.max_y);
    }
}

//c highest a lowest? Keeping track of all this flipping is painful
fn barycentric_coords(a : Point3<i32>, b : Point3<i32>, c : Point3<i32>, p : Point3<i32>) -> Vector3<f32> {
    let x_component : Vector3<f32> = Vector3::new((c.x - a.x) as f32, (b.x - a.x) as f32, (a.x - p.x) as f32);
    let y_component : Vector3<f32> = Vector3::new((c.y - a.y) as f32, (b.y - a.y) as f32, (a.y - p.y) as f32);
    let u : Vector3<f32> = x_component.cross(& y_component);
    //if u.z == 0.0 { //Unexpectedly this actually cuts out valid triangles, I must be missing something.
    //    return Vector3::new(-1.0, 1.0, 1.0)
    //} else {
    Vector3::new(1.0-(u.x+u.y)/u.z, u.y/u.z, u.x/u.z)//1-infinity is negative so this'll be caught later anyways.
    //}
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

fn clown_render(mesh: & Mesh, img: & mut RgbImage, color: Rgb<u8>, scale: f32) {
    let mut rng = rand::thread_rng();
    let get_coord = | index: usize, dir: usize, f: usize | ((mesh.positions[3 * (mesh.indices[3 * f + index])as usize + dir]+1.0)*scale/2.0) as i32;
    for f in 0..mesh.indices.len() / 3 {
        println!("{}", f);
        let x0 = get_coord(0,Direction::X as usize, f);
        let x1 = get_coord(1,Direction::X as usize, f);
        let x2 = get_coord(2,Direction::X as usize, f);
        let y0 = get_coord(0,Direction::Y as usize, f);
        let y1 = get_coord(1,Direction::Y as usize, f);
        let y2 = get_coord(2,Direction::Y as usize, f);
        let z0 = get_coord(0,Direction::Z as usize, f);
        let z1 = get_coord(1,Direction::Z as usize, f);
        let z2 = get_coord(2,Direction::Z as usize, f);
        let v0 : Point3<i32> = Point3::new(x0, y0, z0);
        let v1 : Point3<i32> = Point3::new(x1, y1, z1);
        let v2 : Point3<i32> = Point3::new(x2, y2, z2);
        triangle(v0, v1, v2, img, Rgb([rng.gen(), rng.gen(), rng.gen()]));
    }
}

fn basic_render(mesh: & Mesh, img: & mut RgbImage, color: Rgb<u8>, scale: f32) {
    // for (int i=0; i<model->nfaces(); i++) { 
    // std::vector<int> face = model->face(i); 
    // Vec2i screen_coords[3]; 
    // for (int j=0; j<3; j++) { 
    //     Vec3f world_coords = model->vert(face[j]); 
    //     screen_coords[j] = Vec2i((world_coords.x+1.)*width/2., (world_coords.y+1.)*height/2.); 
    // } 
    // triangle(screen_coords[0], screen_coords[1], screen_coords[2], image, TGAColor(rand()%255, rand()%255, rand()%255, 255)); 
    let light_dir : Vector3<f32> = Vector3::new(0.0,0.0,-0.5);
    let mut rng = rand::thread_rng();
    let get_coord = | index: usize, dir: usize, f: usize | ((mesh.positions[3 * (mesh.indices[3 * f + index])as usize + dir]+1.0)*scale/2.0) as i32;
    let world_coord = | index: usize, f: usize | Vector3::new(mesh.positions[3*(mesh.indices[3*f+index]) as usize + (Direction::X as usize)] as f32,
                                                                        mesh.positions[3*(mesh.indices[3*f+index]) as usize + (Direction::Y as usize)] as f32,
                                                                        mesh.positions[3*(mesh.indices[3*f+index]) as usize + (Direction::Z as usize)] as f32);
    for f in 0..mesh.indices.len() / 3 {
        println!("{}", f);
        let x0 = get_coord(0,Direction::X as usize, f);
        let x1 = get_coord(1,Direction::X as usize, f);
        let x2 = get_coord(2,Direction::X as usize, f);
        let y0 = get_coord(0,Direction::Y as usize, f);
        let y1 = get_coord(1,Direction::Y as usize, f);
        let y2 = get_coord(2,Direction::Y as usize, f);
        let z0 = get_coord(0,Direction::Z as usize, f);
        let z1 = get_coord(1,Direction::Z as usize, f);
        let z2 = get_coord(2,Direction::Z as usize, f);
        let v0 : Point3<i32> = Point3::new(x0, y0, z0);
        let v1 : Point3<i32> = Point3::new(x1, y1, z1);
        let v2 : Point3<i32> = Point3::new(x2, y2, z2);
        let n : Vector3<f32> = ((world_coord(2,f)-world_coord(0,f)).cross(&(world_coord(1,f)-world_coord(0,f)))).normalize();
        let intensity : f32 = n.dot(&light_dir);
        if intensity > 0.0 {
            triangle(v0, v1, v2, img, Rgb([(intensity*255.0) as u8, (intensity*255.0) as u8, (intensity*255.0) as u8]));
        }
    }
    
}
