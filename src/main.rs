use std::{io::Read, io::Write, net::TcpStream};
mod torrent;

use torrent::Torrent;

struct PeerConnection {
    choked: bool,
    interested: bool,
}

fn main() {
    let torrent = Torrent::from("sample.torrent");

    println!("{:#?}", torrent.get());

    let handshake: Vec<u8> = [19]
        .into_iter()
        .chain("BitTorrent protocol".as_bytes().into_iter().cloned())
        .chain([0, 0, 0, 0, 0, 0, 0, 0].into_iter())
        .chain(torrent.info_hash.as_ref().iter().cloned())
        .chain("12345678901234567892".as_bytes().into_iter().cloned())
        .collect();

    let mut buffer = [0u8; 1024];
    let mut stream = TcpStream::connect("165.232.33.77:51467").unwrap();

    stream.write(&handshake).unwrap();

    let size = stream.read(&mut buffer).unwrap();

    stream.shutdown(std::net::Shutdown::Both).unwrap();

    let got_info_hash = Vec::from(&buffer[28..48]);
    let got_peer_id = Vec::from(&buffer[48..68]);

    println!("{:?}", &buffer[..size]);
    println!("{}", String::from_utf8_lossy(&buffer));

    println!("O info hash: {}", hex::encode(&torrent.info_hash));
    println!("T info hash: {}", hex::encode(&got_info_hash));
    println!("Their peer ID: {}", hex::encode(&got_peer_id))
}
