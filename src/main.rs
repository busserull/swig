use std::env;
use std::fs;
use std::path::Path;

mod torrent;

use torrent::Torrent;

fn main() {
    let torrent = Torrent::from("sample.torrent");

    println!("{:#?}", torrent.get());
}
