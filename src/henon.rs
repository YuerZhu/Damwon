extern crate image;

use image::{DynamicImage, GenericImageView, GenericImage,  RgbaImage,  imageops, Pixel, Rgba};
use crate::chunks;

//Key is the key needed to encrypt/decrypt a file, without all four elements of the
//key it is IMPOSSIBLE to decrypt an image, making it extra secure
trait Trait: Sync {}
pub struct Key{
    pub x: f64,
    pub y: f64,
    pub horizontal_chunks: u32,
    pub vertical_chunks: u32,
}
impl Trait for Key{}


//dec converts a vector of 1's and 0's with length 8 into an integer. This is a clever
//way to ensure that the pixel values <= 255 while also making it reversible
pub fn dec(bitSequence: Vec<i32>) -> i32{
    let mut decimal = 0;
    for bit in bitSequence{
        decimal = decimal * 2 + (bit as i32);
    }
    return decimal
}

//The HenonMap is used to genrate a matrix of numbers with which each pixel will get
//transformed by its corresponding indice. This uses the Henon Encryption scheme as well
//as bits -> int using the dec() function above
pub fn genHenonMap( dimensionX: u32, dimensionY: u32, key: &Key) -> Vec<Vec<i32>>{
    let mut x: f64 = key.x;
    let mut y: f64 = key.y;
    let seqSize = dimensionX*dimensionY*8;
    let mut bitSequence = Vec::new();
    let mut byteArray = Vec::new();
    let mut TImageMatrix = Vec::new();
    for i in 0..seqSize{
        let xN = y + 1.0 - 1.4 * x*x;
        let yN = 0.3 * x;

        x = xN;
        y = yN;
        
        let mut bit = 0;

        if xN<= 0.4 {
            bit = 0
        }
        else {
            bit = 1
        }

        bitSequence.push(bit);

        if i % 8 == 7 {
            let decimal = dec(bitSequence);
            byteArray.push(decimal);
            bitSequence = Vec::new();
        }

        let byteArraySize = dimensionX*8;
        if i % byteArraySize == byteArraySize - 1{
            TImageMatrix.push(byteArray);
            byteArray = Vec::new();
        }        
    }
    return TImageMatrix;
}

//The henonEncrypt simply takes the henonMap and applies the transformation to every pixel
//NOTE: the transparency of the pixels never changes which may giveaway the shape of images with 
//transparent backgrounds or chunks
pub fn henonEncrypt(img: image::RgbaImage, key: &Key) -> image::DynamicImage {
    let imageMatrix = img;
    let dimensionX = imageMatrix.width();
    let dimensionY = imageMatrix.height();
    
    let transformationMatrix = genHenonMap(dimensionX, dimensionY, key);

    let mut resultantMatrix: RgbaImage = RgbaImage::new(dimensionX, dimensionY);

    for i in 0 as usize..dimensionY as usize{
        for j in 0 as usize..dimensionX as usize{
            let mult = transformationMatrix[i][j];
            let channels = imageMatrix.get_pixel(j as u32, i as u32).channels();
            //The ^ (XOR) operator is used below. This provides the ability to transform beyond
            //human recognition while also maintaining the ability to be redone by simply calling the
            //XOR operator again
            let pixel: Rgba<u8> = Pixel::from_channels(mult as u8 ^ channels[0], mult as u8 ^ channels[1], mult as u8 ^ channels[2],channels[3]);
            
            resultantMatrix.put_pixel(j as u32, i as u32, pixel);
        }
    }
    return DynamicImage::ImageRgba8(resultantMatrix);
}

//In Henon decryption, the process is just applying the encryption again with the same key. One thing to 
//note about this function is that the chunks must be given to it in order for it to decrypt properly. Whereas
//the henonEncrypt function operates on chunks, henonDecrypt operates on the full image. As a result, the 
//encryption can be done with multithreading, but the decryption must be done sequentially and slowly.
//NOTE: when we finish multithreading make this operate on one chunk only
pub fn henonDecrypt(img: image::RgbaImage, key: &Key) -> image::DynamicImage {
    let mut chunks = chunks::split_into_chunks(&mut DynamicImage::ImageRgba8(img) , key.horizontal_chunks, key.vertical_chunks).unwrap();
    for v in 0..key.vertical_chunks as usize {
        for h in 0..key.horizontal_chunks as usize {
            chunks[v][h] = henonEncrypt(chunks[v][h].to_rgba8(), key);
        }
    }
    return chunks::combine_from_chunks(chunks, key.horizontal_chunks, key.vertical_chunks).unwrap();
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
