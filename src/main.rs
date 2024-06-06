use std::env;
use std::fs;
use std::path::Path;

mod bencode;

use bencode::Bencoded;

/*
struct Torrent {
    announce: String,
    name: String,
    byte_size: usize,
    piece_size: usize,
    pieces: usize,
}

impl Torrent {
    pub fn from<P: AsRef<Path>>(path: P) -> Self {
        let raw_content = fs::read(path).expect("cannot read torrent file");
        let bencoded = Bencoded::parse(&raw_content);
    }
}
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded =
            Bencoded::parse(encoded_value.as_bytes()).expect("cannot decode invalid input");

        println!("{}", decoded);
    } else {
        println!("unknown command: {}", args[1])
    }
}
