use std::{io::Read, io::Write, net::TcpStream};
mod torrent;

use torrent::{PeerId, PeerMessage, Torrent};

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

    let mut peers = peer_list.connect(1);

    let peer = peers.first_mut().expect("No peers in peer list");

    let result = peer.download(&torrent, 0);

    println!("{:?}", result);
}
