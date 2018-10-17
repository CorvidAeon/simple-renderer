extern crate image;

use image::{RgbImage, Rgb};

fn main() {
    let mut img = RgbImage::new(100,100);
    line(13,20,80,40,& mut img,Rgb([255,255,255]));
    line(20,13,40,80,& mut img,Rgb([255,0,0]));
    line(80,40,13,20,& mut img,Rgb([255,0,0]));
    img = image::imageops::flip_vertical(& img);
    img.save("test.png").unwrap();
    //println!("Hello, world!");
}

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
    let mut x = mx0;
    while x<=mx1 {
        let t: f32 = (x-mx0) as f32/(mx1-mx0) as f32;
        let y: u32 = (my0 as f32*(1.0-t) + (my1 as f32*t)) as u32;
        if steep {
            img.put_pixel(y, x, color); //detranspose transposed version
        } else {
            img.put_pixel(x, y, color);
        }
        
        x+=1;
    }
}