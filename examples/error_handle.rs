use std::error::Error;
use std::fs::File;
use std::io::Read;

use rand::Rng;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e)),
    };
    let f = File::open("hello.txt")?;
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("file not exist");
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s);
    return Ok(());
}