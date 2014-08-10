extern crate squint;

use std::io;
use std::os;

fn main() {
    let file = &os::args()[1];
    println!("loading squintfile {}", file);

    let reader = io::File::open(&Path::new(file.as_slice())).ok().unwrap();
    let ref mut reader = io::BufferedReader::new(reader);
    let squee = squint::parse(reader);
    println!("squintax tree: {}", squee);
}
