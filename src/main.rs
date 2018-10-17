extern crate image;

use image::{RgbImage, Rgb};

fn main() {
    let mut img = RgbImage::new(100,100);
    line(13,20,80,40,& mut img,Rgb([255,255,255]));
    line(20,13,40,80,& mut img,Rgb([255,0,0]));
    line(80,40,13,20,& mut img,Rgb([255,0,0]));
    img = image::imageops::flip_vertical(& img);
    img.save("test.png").unwrap();
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
    let dx = mx1-mx0;
    let dy = my1-my0;
    let derror2 = dy.pow(2);
    let mut error2 = 0;
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
            error2 -= dx.pow(2);
        }       
        x+=1;
    }
}