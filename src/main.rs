extern crate image;

pub mod functions;
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, RgbImage, RgbaImage, SubImage, imageops};

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let mut img =  image::open("rust.png").unwrap();    

    let (vert, horiz) = (3,3);

    let vec : Vec<Vec<RgbaImage>> = functions::split_into_chunks(&mut img, vert, horiz).unwrap();

    // prints out all chunks CREATES A LOT OF FILES
    for h in 0..vert as usize{
        for v in 0..horiz as usize{
            let s =  "ImageBin/".to_string() + &*format!("{}{}", h, v) + ".png";
            vec[h][v].save( s).unwrap();
        }
    }
    //
    
}


