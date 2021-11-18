extern crate image;

pub mod functions;
pub mod thread;

use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, RgbImage, RgbaImage, SubImage, imageops};

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.


    /* Chunks tutorial
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
    */
    /* Encrypt tutorial
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let mut img = image::open("bun.jpg").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());
    let pixel = img.get_pixel(300, 300);
    // The color method returns the image's `ColorType`.
    println!("{:?}", pixel.0[0]);
    img = encrypt(img);
    img.save("encr.png").unwrap();
    img = decrypt(img);
    // img = encrypt(img);
    // Write the contents of this image to the Writer in PNG format.
    img.save("decr.png").unwrap();
    */
}


