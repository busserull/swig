use std::{io::Read, io::Write, net::TcpStream};
mod torrent;

use torrent::{PeerConnection, PeerId, Torrent};

enum MessageType {
    Choke = 0,
    Unchoke = 1,
    Interested = 2,
    NotInterested = 3,
    Have = 4,
    Bitfield = 5,
    Request = 6,
    Piece = 7,
    Cancel = 8,
}

struct ConnectedPeer {
    choked: bool,
    interested: bool,
    stream: TcpStream,
    buffer: Vec<u8>,
}

/*
impl ConnectedPeer {
    fn new(address: &str, our_id: &str, info_hash: &[u8]) -> Self {
        let mut stream = TcpStream::connect(address).
    }
}
*/

fn main() {
    let id = PeerId::new();

    let torrent = Torrent::from("sample.torrent");

    let peer_list = torrent.get_peer_list(id);

    let peers = peer_list.connect(1);

    for peer in peers {
        println!("{:?}", peer);
    }
}
