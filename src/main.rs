extern crate image;
pub mod threading;
pub mod henon;
pub mod chunks;
use std::env;

static KEY: henon::Key = henon::Key {
    x: 0.1,
    y: 0.2,
    horizontal_chunks: 3,
    vertical_chunks: 3,
};

trait Trait: Sync {}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        return Err("Usage: [encrypt/decrypt] [input file path] [output file path]".to_string());
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
        return Err("One or more of your arguments were incorrect. Usage: [encrypt/decrypt] [input file path] [output file path]".to_string());
    }
    Ok(())
}


