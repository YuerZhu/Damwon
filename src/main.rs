extern crate image;

use image::GenericImageView;
use image::GenericImage;

fn main() {
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