use std::thread::JoinHandle;
use std::thread;

pub fn multi_thread_encrypter(num_threads: usize, Image* og_image)-> Vec<(JoinHandle<()>, Receiver<Image*>)> {
  let mut handles:Vec<(JoinHandle<()>, Receiver<Image*>)> = Vec::new();
  for i in 0..num_threads{
    let images:Vec<Image*> = image_splitter(og_image,num_threads); //???why does it have to be in the for loop???
    let (tx,rx) = mpsc::channel();
    let jh:thread::JoinHandle<()> = thread::spawn(move ||{
        let myimage:&Vec<String> = &chunks[i].clone();
        tx.send(encrypt(myimage).unwrap());
    });
    handles.push((jh,rx));
  }
  handles
}

pub fn multi_thread_decrypter(num_threads: usize, Image* og_image)-> Vec<(JoinHandle<()>, Receiver<Image*>)> {
  let mut handles:Vec<(JoinHandle<()>, Receiver<Image*>)> = Vec::new();
  for i in 0..num_threads{
    let images:Vec<Image*> = image_splitter(og_image,num_threads); //???why does it have to be in the for loop???
    let (tx,rx) = mpsc::channel();
    let jh:thread::JoinHandle<()> = thread::spawn(move ||{
        let myimage:&Vec<String> = &chunks[i].clone();
        tx.send(decrypt(myimage).unwrap());
    });
    handles.push((jh,rx));
  }
  handles
}
