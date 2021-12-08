extern crate threadpool;
use threadpool::ThreadPool;
use std::sync::{mpsc};

use crate::{henon, chunks};
use image::{DynamicImage};

// Splits image into chunks and spawns corresponding threads to encrypt each chunk
// @param `key` - key to generate Henon map
// @param `img` - image to encrypt
// 
// @return encrypted image
pub fn multi_thread_encrypter(key: &'static henon::Key, mut img: &mut DynamicImage)-> Result<DynamicImage, ()> {
  let chnks = chunks::split_into_chunks(&mut img, key.horizontal_chunks, key.vertical_chunks).unwrap();
  
  let pool = ThreadPool::new(key.horizontal_chunks as usize*key.vertical_chunks as usize);
  let (tx,rx) = mpsc::channel();
  for h in 0 as usize..key.horizontal_chunks as usize{
    
    for v in 0..key.vertical_chunks as usize {
      let tx = tx.clone();      
      let x = chnks[v][h].to_rgba8();
      pool.execute( move || {
        tx.send((h, v, henon::henon_encrypt(x , key)));
      });
    }
    }
  let mut vec: Vec<Vec<DynamicImage>> = chnks;
  for _h in 0 as usize..key.horizontal_chunks as usize *key.vertical_chunks as usize{
    let (x, y, chunk) = rx.recv().unwrap();
    vec[y][x] = chunk;
  }
  chunks::combine_from_chunks(vec, key.horizontal_chunks, key.vertical_chunks)
}

// Splits image into chunks and spawns corresponding threads to decrypt each chunk
// @param `key` - key to generate Henon map
// @param `img` - image to decrypt
// 
// @return decrypted image
pub fn multi_thread_decrypter(key: &'static henon::Key, img: &mut DynamicImage)-> Result<DynamicImage, ()> {
  return Ok(multi_thread_encrypter(key, img).unwrap());
}
