use std::env;
use std::fs;
use std::path::Path;

mod torrent;

use torrent::Torrent;

fn main() {
    // let raw_content = fs::read("sample.torrent").expect("cannot read torrent file");
    // let bencoded = Bencoded::parse(&raw_content).expect("cannot parse bencoded data");

    // println!("{}", bencoded);

    let torrent = Torrent::from("sample.torrent");

    println!("{:#?}", torrent);
}
