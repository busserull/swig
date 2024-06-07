use std::env;
use std::fs;
use std::path::Path;

mod bencode;

use bencode::Bencoded;

#[derive(Debug)]
struct Torrent {
    announce: String,
    // name: String,
    byte_size: usize,
    // piece_size: usize,
    // pieces: usize,
}

impl Torrent {
    pub fn from<P: AsRef<Path>>(path: P) -> Self {
        let raw_content = fs::read(path).expect("cannot read torrent file");
        let bencoded = Bencoded::parse(&raw_content).expect("cannot parse bencoded data");

        let announce = if let Some(Bencoded::Bstr(url)) = get(&bencoded, "announce") {
            String::from_utf8_lossy(&url).to_string()
        } else {
            todo!();
        };

        let info = get(&bencoded, "info").unwrap();

        println!("Info: {}", info);

        let byte_size = 1;

        Self {
            announce,
            byte_size,
        }
    }
}

fn get(bencoded: &Bencoded, key: &str) -> Option<Bencoded> {
    let key = Bencoded::Bstr(Vec::from(key.as_bytes()));

    if let Bencoded::Dict(pairs) = bencoded {
        pairs
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v.clone())
    } else {
        None
    }
}

fn main() {
    // let raw_content = fs::read("sample.torrent").expect("cannot read torrent file");
    // let bencoded = Bencoded::parse(&raw_content).expect("cannot parse bencoded data");

    let torrent = Torrent::from("sample.torrent");

    println!("{:#?}", torrent);
}
