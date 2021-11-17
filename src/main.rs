extern crate image;

use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, RgbaImage};

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