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
    let mut x = x0;
    while x<=x1 {
        let t: f32 = (x-x0) as f32/(x1-x0) as f32;
        let y: u32 = (y0 as f32*(1.0-t) + (y1 as f32*t)) as u32;
        img.put_pixel(x, y, color);
        x+=1;
    }
}