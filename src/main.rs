extern crate image;

use image::{RgbImage, Rgb};

fn main() {
    let mut img = RgbImage::new(100,100);
    line(13,20,80,40,& mut img,Rgb([255,255,255]));
    img = image::imageops::flip_vertical(& img);
    img.save("test.png").unwrap();
    //println!("Hello, world!");
}

fn line(x0: u32, y0:u32,x1:u32,y1:u32,img: & mut RgbImage,color: Rgb<u8>){
    let mut t =0.0;
    while t<=1.0 {
        let x = (x0 as f32*(1.0-t) + x1 as f32*t) as u32;
        let y = (y0 as f32*(1.0-t) + y1 as f32*t) as u32;
        img.put_pixel(x, y, color);
        t+=0.01;
    }
}