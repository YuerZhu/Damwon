extern crate image;

use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, RgbImage, RgbaImage, SubImage, imageops};

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