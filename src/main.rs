extern crate image;

use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, RgbaImage, DynamicImage};

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let mut img =  image::open("rust.png").unwrap();    

    let mut ret: RgbaImage = ImageBuffer::new(img.dimensions().0, img.dimensions().1);    

    for x in 0..img.dimensions().0{
        for y in 0..img.dimensions().1 {
            ret.put_pixel(x, y, img.get_pixel(x, y));
            ret.get_pixel_mut(x, y).invert();
        }    
    }

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    ret.save("inverted.png").unwrap();
}

fn split_into_chunks(img: DynamicImage, vert: u32, horiz: u32) -> Result<Vec<Vec<RgbaImage>>, ()> {
    let pixel_width = (img.dimensions().0 / horiz) as u32;
    let pixel_height = (img.dimensions().1 / vert) as u32;
    let mut ret: Vec<Vec<RgbaImage>> = Vec::new();
    for  v in 0..(vert-1) {
        let mut row: Vec<RgbaImage> = Vec::new();        
        for h in 0..(horiz-1) {
            let mut chunk: RgbaImage = ImageBuffer::new(pixel_width, pixel_height);
            for x in 0..pixel_height{
                for y in 0..pixel_width{
                    chunk.put_pixel(x, y, img.get_pixel(pixel_width*h + x, pixel_height*v +y));
                }    
            }
        row.push(chunk);
        }
        ret.push(row);        
    }
    for v in 0..(vert - 1){
        let mut chunk: RgbaImage = ImageBuffer::new((img.dimensions().0 -pixel_width*(horiz-1)), pixel_height);
            for x in 0..(img.dimensions().0 -pixel_width*(horiz-1)){
                for y in 0..pixel_width{
                    chunk.put_pixel(x, y, img.get_pixel(pixel_width*(horiz-1) + x, pixel_height*v +y));
                }    
            }
        ret[h].push(chunk);
    }
    let mut row: Vec<RgbaImage> = Vec::new();
    for h in 0..(horiz-1) {        
        let mut chunk: RgbaImage = ImageBuffer::new(pixel_width, (img.dimensions().1 -pixel_height*(vert-1)));
            for x in 0..pixel_width{
                for y in 0..(img.dimensions().1 -pixel_height*(vert-1)){
                    chunk.put_pixel(x, y, img.get_pixel(pixel_width*h + x, pixel_height*(vert-1) +y));
                }    
            }
        row.push(chunk);
    }
    ret.push(row);
    let mut chunk: RgbaImage = ImageBuffer::new((img.dimensions().0 -pixel_width*(horiz-1)), (img.dimensions().1 -pixel_height*(vert-1)));
    for x in 0..(img.dimensions().0 -pixel_width*(horiz-1)){
        for y in 0..(img.dimensions().1 -pixel_height*(vert-1)){
            chunk.put_pixel(x, y, img.get_pixel(pixel_width*(horiz-1) + x, pixel_height*(vert-1) +y));
        }    
    }
    ret[vert].push(chunk);
    
    return Ok(ret);
} 