mod torrent;

use torrent::{PeerId, Torrent};

fn main() {
    let id = PeerId::new();

    let torrent = Torrent::from("sample.torrent");

    let peer_list = torrent.get_peer_list(id);

    let mut peers = peer_list.connect(1);

    let peer = peers.first_mut().expect("No peers in peer list");

    let result = peer.download(&torrent, 0);

    match result {
        Ok(bytes) => println!("{}", String::from_utf8_lossy(&bytes)),
        Err(e) => println!("Error: {:?}", e),
    }
}
