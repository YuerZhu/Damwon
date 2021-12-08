extern crate image;
<<<<<<< HEAD
pub mod thread;
=======
pub mod threading;
>>>>>>> 243d1e7e19016aac939f39ae67f532bc1ef69cd1
pub mod henon;
pub mod chunks;
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, RgbImage, RgbaImage, SubImage, imageops};
use std::env;

static KEY: henon::Key = henon::Key {
    x: 0.1,
    y: 0.2,
    horizontal_chunks: 3,
    vertical_chunks: 3,
};

trait Trait: Sync {}

fn main() -> Result<(), ()> {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        return Err(());
    }
    let action = &args[1];
    let input = &args[2];
    let output = &args[3];
    if action == "encrypt" { // cargo run encrypt InputImages/Forest.jpg ImageBin/combined.png
        let mut img =  image::open(input).unwrap();   
        threading::multi_thread_encrypter(&KEY, &mut img).unwrap().save(output).unwrap();
    }else if action == "decrypt" { // cargo run decrypt ImageBin/combined.png ImageBin/decrypted.png
        let mut enc =  image::open(input).unwrap();
        threading::multi_thread_decrypter(&KEY, &mut enc).unwrap().save(output).unwrap();
    }else{
        return Err(());
    }

    Ok(())
    // chunks::combine_from_chunks(chunks::split_into_chunks(&mut img, 3,3).unwrap(),3,3).unwrap().save(s).unwrap();
    // let vec : Vec<Vec<DynamicImage>> = chunks::split_into_chunks(&mut img, key.horizontal_chunks, key.vertical_chunks).unwrap();
    // let mut fin: Vec<Vec<DynamicImage>>  = Vec::new();
    // // prints out all chunks CREATES A LOT OF FILES
    // for h in 0..key.horizontal_chunks as usize{
    //     let mut add: Vec<DynamicImage> = Vec::new();
    //     for v in 0..key.vertical_chunks as usize{
    //         let s =  "ImageBin/".to_string() + &*format!("{}{}", v, h) + ".png";
    //         let temp = henon::henonEncrypt(vec[v][h].to_rgba8(), &key);
    //         temp.save( s).unwrap();
    //         if fin.len() < v + 1{
    //             fin.push(Vec::new());
    //         }
    //         fin[v].push(temp);
    //     }
    // }
    // let s =  "ImageBin/combined.png";
    // let d =  "ImageBin/decrypted.png";
    // //saves the cumulative encryption with all chunks combined
    // let x = chunks::combine_from_chunks(fin, key.horizontal_chunks, key.vertical_chunks).unwrap();
    // x.save(s).unwrap();
    // //saves the decryption of the entire image
    // henon::henonDecrypt(x.to_rgba8(), &key).save(d).unwrap();
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


