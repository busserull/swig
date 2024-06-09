mod torrent;

use std::io::Write;

use torrent::{PeerId, Torrent};

fn main() {
    let id = PeerId::new();

    let torrent = Torrent::from("sample.torrent");
    println!("{:#?}", torrent);

    let mut single_file = torrent.create_single_payload();

    let peer_list = torrent.get_peer_list(id);

    let mut peers = peer_list.connect(1);

    let peer = peers.first_mut().expect("No peers in peer list");

    for piece in 0..torrent.piece_count() {
        match peer.download(&torrent, piece) {
            Ok(bytes) => {
                single_file.write(&bytes).ok();
            }

            Err(e) => println!("Error: {:?}", e),
        }
    }

    println!("Download complete");
}
