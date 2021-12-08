extern crate threadpool;
use threadpool::ThreadPool;
use std::{thread, thread::{JoinHandle}};
use std::sync::{mpsc, mpsc::{Receiver}};

use crate::{henon, chunks};
use image::{DynamicImage, GenericImageView, GenericImage,  RgbaImage,  imageops, Pixel, Rgba};

// pub fn multi_thread_encrypter(key: &'static henon::Key, mut img: &mut DynamicImage)-> Result<DynamicImage, ()> {
//   let imageMatrix = img.to_rgba8();
//   let henonMap = henon::genHenonMap(img.width(), img.height(), key);
//   let pool = ThreadPool::new(key.horizontal_chunks as usize*key.vertical_chunks as usize);
//   let (tx,rx) = mpsc::channel();
//   for h in 0 as usize..img.width() as usize{    
//     for v in 0..img.height() as usize {
//       let tx = tx.clone();      
//       // let images:Vec<DynamicImage> = image_splitter(og_image,num_threads); //???why does it have to be in the for loop???  
//       let channels1 = imageMatrix.get_pixel(h as u32, v as u32).channels()[0];
//       let channels2 = imageMatrix.get_pixel(h as u32, v as u32).channels()[1];
//       let channels3 = imageMatrix.get_pixel(h as u32, v as u32).channels()[2];
//       let channels4 = imageMatrix.get_pixel(h as u32, v as u32).channels()[3];
//       let mult = henonMap[v][h];    
//       pool.execute( move || {        
//         tx.send((h, v, Pixel::from_channels(mult as u8 ^ channels1, mult as u8 ^ channels2, mult as u8 ^ channels3,channels4)));
//       });
//     }
//     }
//     let mut ret = RgbaImage::new(img.width(), img.height());
//   for h in 0 as usize..img.height() as usize *img.width() as usize{
//     let (x, y, pixel) = rx.recv().unwrap();
//     ret.put_pixel(x as u32 ,y as u32, pixel);
//   }
//   Ok(DynamicImage::ImageRgba8(ret))
// }

pub fn multi_thread_decrypter(key: &'static henon::Key, mut img: &mut DynamicImage)-> Result<DynamicImage, ()> {
  return Ok(multi_thread_encrypter(key, img).unwrap());
}



pub fn multi_thread_encrypter(key: &'static henon::Key, mut img: &mut DynamicImage)-> Result<DynamicImage, ()> {
  let mut chnks = chunks::split_into_chunks(&mut img, key.horizontal_chunks, key.vertical_chunks).unwrap();
  
  let pool = ThreadPool::new(key.horizontal_chunks as usize*key.vertical_chunks as usize);
  let (tx,rx) = mpsc::channel();
  for h in 0 as usize..key.horizontal_chunks as usize{
    
    for v in 0..key.vertical_chunks as usize {
      let tx = tx.clone();      
      let x = chnks[v][h].to_rgba8();
      pool.execute( move || {
        tx.send((h, v, henon::henonEncrypt(x , key)));
      });
    }
    }
  let mut vec: Vec<Vec<DynamicImage>> = chnks;
  for h in 0 as usize..key.horizontal_chunks as usize *key.vertical_chunks as usize{
    let (x, y, chunk) = rx.recv().unwrap();
    vec[y][x] = chunk;
  }
  chunks::combine_from_chunks(vec, key.horizontal_chunks, key.vertical_chunks)
}