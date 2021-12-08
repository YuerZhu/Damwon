extern crate image;
extern crate threadpool;
use threadpool::ThreadPool;
use std::{thread, thread::{JoinHandle}};
use std::sync::{mpsc, mpsc::{Receiver}};
use crate::henon::Key;

use image::{DynamicImage, GenericImageView, GenericImage,  RgbaImage,  imageops, Pixel, Rgba};

//split_into_chunks splits an image into a given amount of horizontal and vertical chunks with chunks
//being as evenly sized as possible. This is designed to make multithreading encryption possible.
pub fn split_into_chunks(mut img: &mut DynamicImage, horiz: u32, vert: u32) -> Result<Vec<Vec<DynamicImage>>, ()> {
    let (width, height) = img.dimensions();
    let pixel_width = ((width as f32/ horiz as f32) + 0.5) as u32;
    let pixel_height = ((height as f32/ vert as f32) + 0.5) as u32;
    //Adds all internal subimage chunks (chunks that are full (not edges)) 
    let mut ret: Vec<Vec<DynamicImage>> = Vec::new();
    for  v in 0..vert {
        let mut row: Vec<DynamicImage> = Vec::new();        
        for h in 0..horiz {
            let mut curwidth = pixel_width;
            let mut curheight = pixel_height;
            if h == horiz-1 {
                curwidth = width - (pixel_width*(horiz-1));
            }
            if v == vert-1 {
                curheight = height- (pixel_height*(vert-1));
            }
            let mut chunk = imageops::crop( img, h*pixel_width, v*pixel_height, curwidth, curheight).to_image();
             
        row.push(DynamicImage::ImageRgba8(chunk));
        }
        ret.push(row);        
    }
    return Ok(ret);
}


//combine_from_chunks is the inverse of split_into_chunks and serves the sole purpose of recombining
//the chunks into one cumulative image.
pub fn combine_from_chunks(subimgs: Vec<Vec<DynamicImage>>, horiz: u32, vert: u32) -> Result< DynamicImage, ()> {
    let mut total_width = 0;
    let mut total_height = 0;
    for i in 0 as usize..horiz as usize{
        total_width += subimgs[0][i].width();
    }
    for i in 0 as usize..vert as usize{
        total_height += subimgs[i][0].height();
    }
    let (width, height) = subimgs[0][0].dimensions();
    let mut img = RgbaImage::new(total_width, total_height);
    for h in 0 as usize..horiz as usize {
        for v in 0 as usize..vert as usize{
            for a in 0 as u32..subimgs[v][h].width(){
                for b in 0 as u32..subimgs[v][h].height(){
                    img.put_pixel((h as u32)*width + a , (v as u32)*height + b , subimgs[v][h].get_pixel(a, b));
                }
            }
        }
    }
    return Ok(DynamicImage::ImageRgba8(img));
}