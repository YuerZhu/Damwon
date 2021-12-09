extern crate image;
pub mod threading;
pub mod henon;
pub mod chunks;
use std::env;



trait Trait: Sync {}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        return Err("Usage: [encrypt/decrypt] [input file path] [output file path] [key 1] [key 2]".to_string());
    }
    let action = &args[1];
    let input = &args[2];
    let output = &args[3];
    let key: henon::Key<> = henon::Key {
        x: args[4].parse().unwrap(),
        y: args[5].parse().unwrap(),
        horizontal_chunks: 3,
        vertical_chunks: 3,
    };
    if action == "encrypt" { // cargo run encrypt InputImages/Forest.jpg ImageBin/combined.png 0.1  0.3
        let mut img =  image::open(input).unwrap();   
        threading::multi_thread_encrypter(key, &mut img).unwrap().save(output).unwrap();
    }else if action == "decrypt" { // cargo run decrypt ImageBin/combined.png ImageBin/decrypted.png 0.1 0.3
        let mut enc =  image::open(input).unwrap();
        threading::multi_thread_decrypter(key, &mut enc).unwrap().save(output).unwrap();
    }else{
        return Err("One or more of your arguments were incorrect. Usage: [encrypt/decrypt] [input file path] [output file path] [key 1] [key 2]".to_string());
    }
    Ok(())
}


