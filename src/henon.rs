extern crate image;

use image::{DynamicImage, RgbaImage, Pixel, Rgba};
use crate::chunks;

//Key is the key needed to encrypt/decrypt a file, without all four elements of the
//key it is IMPOSSIBLE to decrypt an image, making it extra secure
trait Trait: Sync {}
#[derive(Copy, Clone)]
pub struct Key{
    pub x: f64,
    pub y: f64,
    pub horizontal_chunks: u32,
    pub vertical_chunks: u32,
}
impl Trait for Key{}


//dec converts a vector of 1's and 0's with length 8 into an integer. This is a clever
//way to ensure that the pixel values <= 255 while also making it reversible
pub fn dec(bit_sequence: Vec<i32>) -> i32{
    let mut decimal = 0;
    for bit in bit_sequence{
        decimal = decimal * 2 + (bit as i32);
    }
    return decimal
}

//The HenonMap is used to genrate a matrix of numbers with which each pixel will get
//transformed by its corresponding indice. This uses the Henon Encryption scheme as well
//as bits -> int using the dec() function above
// @param `dimension_x`, `dimension_y` - the dimensions of the image
// @param `key` - a Key that creates a unique Henon map
// 
// @return the generated Henon map
pub fn gen_henon_map( dimension_x: u32, dimension_y: u32, key: Key) -> Vec<Vec<i32>>{
    let mut x: f64 = key.x;
    let mut y: f64 = key.y;
    let seq_size = dimension_x*dimension_y*8;
    let mut bit_sequence = Vec::new();
    let mut byte_array = Vec::new();
    let mut t_image_matrix = Vec::new();
    for i in 0..seq_size{
        let x_n = y + 1.0 - 1.4 * x*x;
        let y_n = 0.3 * x;

        x = x_n;
        y = y_n;
        
        let bit;

        if x_n<= 0.4 {
            bit = 0
        }
        else {
            bit = 1
        }

        bit_sequence.push(bit);

        if i % 8 == 7 {
            let decimal = dec(bit_sequence);
            byte_array.push(decimal);
            bit_sequence = Vec::new();
        }

        let byte_array_size = dimension_x*8;
        if i % byte_array_size == byte_array_size - 1{
            t_image_matrix.push(byte_array);
            byte_array = Vec::new();
        }        
    }
    return t_image_matrix;
}

//The henonEncrypt simply takes the henonMap and applies the transformation to every pixel
//NOTE: the transparency of the pixels never changes which may giveaway the shape of images with 
//transparent backgrounds or chunks
// @param `img` - the image to be encrypted
// @param `key` - the key for the Henon map
// 
// @return the encrypted image
pub fn henon_encrypt(img: image::RgbaImage, key: Key) -> image::DynamicImage {
    let image_matrix = img;
    let dimension_x = image_matrix.width();
    let dimension_y = image_matrix.height();
    
    let transformation_matrix = gen_henon_map(dimension_x, dimension_y, key);

    let mut resultant_matrix: RgbaImage = RgbaImage::new(dimension_x, dimension_y);

    for i in 0 as usize..dimension_y as usize{
        for j in 0 as usize..dimension_x as usize{
            let mult = transformation_matrix[i][j];
            let channels = image_matrix.get_pixel(j as u32, i as u32).channels();
            //The ^ (XOR) operator is used below. This provides the ability to transform beyond
            //human recognition while also maintaining the ability to be redone by simply calling the
            //XOR operator again
            let pixel: Rgba<u8> = Pixel::from_channels(mult as u8 ^ channels[0], mult as u8 ^ channels[1], mult as u8 ^ channels[2],channels[3]);
            
            resultant_matrix.put_pixel(j as u32, i as u32, pixel);
        }
    }
    return DynamicImage::ImageRgba8(resultant_matrix);
}

//In Henon decryption, the process is just applying the encryption again with the same key. One thing to 
//note about this function is that the chunks must be given to it in order for it to decrypt properly. Whereas
//the henonEncrypt function operates on chunks, henonDecrypt operates on the full image. As a result, the 
//encryption can be done with multithreading, but the decryption must be done sequentially and slowly.
//NOTE: when we finish multithreading make this operate on one chunk only
// @param `img` - the encrypted image
// @param `key` - the key for the Henon map
// 
// @return the decrypted image
pub fn henon_decrypt(img: image::RgbaImage, key: Key) -> image::DynamicImage {
    let mut chunks = chunks::split_into_chunks(&mut DynamicImage::ImageRgba8(img) , key.horizontal_chunks, key.vertical_chunks).unwrap();
    for v in 0..key.vertical_chunks as usize {
        for h in 0..key.horizontal_chunks as usize {
            chunks[v][h] = henon_encrypt(chunks[v][h].to_rgba8(), key);
        }
    }
    return chunks::combine_from_chunks(chunks, key.horizontal_chunks, key.vertical_chunks).unwrap();
}
