extern crate image;

use image::{DynamicImage, GenericImageView, GenericImage,  RgbaImage,  imageops};

pub fn split_into_chunks(img: &mut DynamicImage, horiz: u32, vert: u32) -> Result<Vec<Vec<RgbaImage>>, ()> {
    let (width, height) = img.dimensions();
    let pixel_width = (width / horiz) as u32;
    let pixel_height = (height / vert) as u32;
    //Adds all internal subimage chunks (chunks that are full (not edges)) 
    let mut ret: Vec<Vec<RgbaImage>> = Vec::new();
    for  v in 0..vert {
        let mut row: Vec<RgbaImage> = Vec::new();        
        for h in 0..horiz {
            let mut curwidth = pixel_width;
            let mut curheight = pixel_height;
            if h == horiz-1 {
                curwidth = width - (pixel_width*(horiz-1));
            }
            if v == vert-1 {
                curheight = height- (pixel_height*(vert-1));
            }
            let chunk= imageops::crop( img, h*pixel_width, v*pixel_height, curwidth, curheight).to_image();
        row.push(chunk);
        }
        ret.push(row);        
    }
    return Ok(ret);
} 

pub fn encrypt(mut img: image::DynamicImage) -> image::DynamicImage{
    let (width, height) = img.dimensions();
    for i in 0..width{
        for j in 0..height{
            if i%2 == 0 && i<width/2{
                // let inv_pixel = img.get_pixel(width - i - 1, height - j - 1);
                let inv_pixel = img.get_pixel(width - i - 2, j);
                let pixel = img.get_pixel(i, j);
                img.put_pixel(i, j, inv_pixel);
                img.put_pixel(width - i - 2, j, pixel);
            }
            else if j%2 == 0 && j < height/2{
                let inv_pixel = img.get_pixel(i, height - j - 2);
                let pixel = img.get_pixel(i, j);
                img.put_pixel(i, j, inv_pixel); 
                img.put_pixel(i, height - j - 2, pixel); 
            }
            else{
                let mut pixel = img.get_pixel(i, j);
                let tmp = pixel.0[0];
                pixel.0[0] = pixel.0[1];
                pixel.0[1] = pixel.0[2];
                pixel.0[2] = tmp;
                img.put_pixel(i, j, pixel);
            }
        }
    }
    img
}

pub fn decrypt(mut img: image::DynamicImage) -> image::DynamicImage{
    let (width, height) = img.dimensions();
    for i in 0..width{
        for j in 0..height{
            if i%2 == 0 && i<width/2{
                // let inv_pixel = img.get_pixel(width - i - 1, height - j - 1);
                let inv_pixel = img.get_pixel(width - i - 2, j);
                let pixel = img.get_pixel(i, j);
                img.put_pixel(i, j, inv_pixel);
                img.put_pixel(width - i - 2, j, pixel);
            }
            else if j%2 == 0 && j < height/2{
                let inv_pixel = img.get_pixel(i, height - j - 2);
                let pixel = img.get_pixel(i, j);
                img.put_pixel(i, j, inv_pixel); 
                img.put_pixel(i, height - j - 2, pixel); 
            }
            else{
                let mut pixel = img.get_pixel(i, j);
                let tmp = pixel.0[2];
                pixel.0[2] = pixel.0[1];
                pixel.0[1] = pixel.0[0]; 
                pixel.0[0] = tmp;
                img.put_pixel(i, j, pixel);
            }
        }
    }
    img
}