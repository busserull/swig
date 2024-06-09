use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;

mod bencode;
use bencode::Bencoded;

mod sha;
use reqwest::header::EXPIRES;
use sha::Sha1;

mod url;
use url::Url;

pub mod peer_id;
pub use peer_id::PeerId;

#[derive(Debug)]
pub struct Torrent {
    // Announce URL of tracker
    announce: String,

    info_hash: Sha1,

    // Number of bytes in each piece
    piece_length: usize,
    // Sha1 hash values for each piece
    pieces: Vec<Sha1>,
    // "No external peer source"
    private: bool,

    payload: Payload,
}

impl Torrent {
    pub fn from<P: AsRef<Path>>(path: P) -> Self {
        let raw_content = fs::read(path).expect("failed to read torrent file");
        let bencoded = Bencoded::parse(&raw_content).expect("failed parsing bencoding");

        let announce = get_bstr(&bencoded, "announce")
            .map(|url| String::from_utf8(url).expect("Malformed torrent `announce`"))
            .expect("No torrent `announce` entry");

        let info = get_dict(&bencoded, "info").expect("No torrent `info` entry");

        let info_reencoded = Vec::from(&info);
        let info_hash = Sha1::digest(&info_reencoded);

        let name = get_bstr(&info, "name")
            .map(|name| String::from_utf8(name).expect("Torrent `info.name` not valid UTF-8"))
            .expect("No torrent `info.name` entry");

        let piece_length = get_int(&info, "piece length")
            .map(|length| length as usize)
            .expect("No torrent `piece length` entry");

        let pieces = get_bstr(&info, "pieces")
            .map(|sha_string| {
                assert_eq!(
                    sha_string.len() % 20,
                    0,
                    "Torrent `pieces` does not contain a valid multiple of SHA1 digests"
                );
                sha_string.chunks_exact(20).map(Sha1::new_raw).collect()
            })
            .expect("No torrent `pieces` entry");

        let private = get_int(&info, "private")
            .map(|private_flag| private_flag == 1)
            .unwrap_or_default();

        let payload = Payload::new(name, &info);

        Self {
            announce,
            info_hash,
            piece_length,
            pieces,
            private,
            payload,
        }
    }

    pub fn get_peer_list(&self, our_id: PeerId) -> PeerList {
        let bytes_left = match &self.payload {
            Payload::Single { name, length } => *length,
            Payload::Multi { name, files } => 0,
        };

        let url: String = Url::new(&self.announce)
            .with_param("info_hash", self.info_hash)
            .with_param("peer_id", our_id.as_ref())
            .with_param("port", 6881)
            .with_param("uploaded", 0)
            .with_param("downloaded", 0)
            .with_param("left", bytes_left)
            .with_param("compact", 1)
            .into();

        let response = reqwest::blocking::get(url)
            .expect("Cannot contact tracker")
            .bytes()
            .expect("Cannot read bytes of tracker response");

        let bencoded = Bencoded::parse(&response).expect("Cannot parse bencoded tracker response");

        PeerList::new(bencoded, our_id, self.info_hash)
    }
}

#[derive(Debug)]
pub struct PeerList {
    our_id: PeerId,
    expected_info_hash: Sha1,
    interval: usize,
    peers: Vec<PeerAddress>,
}

impl PeerList {
    fn new(response: Bencoded, our_id: PeerId, expected_info_hash: Sha1) -> Self {
        let interval =
            get_int(&response, "interval").expect("No `interval` in tracker response") as usize;

        let peers = get_bstr(&response, "peers")
            .expect("No `peers` in tracker response")
            .chunks_exact(6)
            .map(PeerAddress::new)
            .collect();

        Self {
            our_id,
            expected_info_hash,
            interval,
            peers,
        }
    }

    pub fn connect(&self, max_connections: usize) -> Vec<PeerConnection> {
        self.peers
            .iter()
            .take(max_connections)
            .filter_map(|peer| peer.connect(self.our_id, self.expected_info_hash))
            .collect()
    }
}

struct PeerAddress {
    ip: (u8, u8, u8, u8),
    port: u16,
}

impl PeerAddress {
    fn new(peer_sextet: &[u8]) -> Self {
        let mut port_bytes = [0u8; 2];
        port_bytes.copy_from_slice(&peer_sextet[4..=5]);

        let ip = (
            peer_sextet[0],
            peer_sextet[1],
            peer_sextet[2],
            peer_sextet[3],
        );

        let port = u16::from_be_bytes(port_bytes);

        Self { ip, port }
    }

    fn connect(&self, our_id: PeerId, expected_info_hash: Sha1) -> Option<PeerConnection> {
        let handshake: Vec<u8> = [19]
            .into_iter()
            .chain("BitTorrent protocol".as_bytes().into_iter().cloned())
            .chain([0, 0, 0, 0, 0, 0, 0, 0].into_iter())
            .chain(expected_info_hash.as_ref().iter().cloned())
            .chain(our_id.as_ref().into_iter().cloned())
            .collect();

        let mut buffer = [0u8; 1024];
        let mut stream = TcpStream::connect(self.to_string()).ok()?;

        stream.write(&handshake).ok()?;

        let response_size = stream.read(&mut buffer).ok()?;

        if response_size < 68 {
            return None;
        }

        let got_info_hash = Vec::from(&buffer[28..48]);

        if got_info_hash == expected_info_hash.as_ref() {
            Some(PeerConnection { stream, buffer })
        } else {
            None
        }
    }
}

impl std::fmt::Display for PeerAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}:{}",
            self.ip.0, self.ip.1, self.ip.2, self.ip.3, self.port
        )
    }
}

impl std::fmt::Debug for PeerAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PeerAddress({})", self)
    }
}

#[derive(Debug)]
pub struct PeerConnection {
    stream: TcpStream,
    buffer: [u8; 1024],
}

#[derive(Debug)]
enum Payload {
    Single { name: String, length: usize },
    Multi { name: String, files: Vec<File> },
}

impl Payload {
    fn new(name: String, info: &Bencoded) -> Self {
        let length = get_int(info, "length");
        let files = get_list(info, "files");

        match (length, files) {
            (Some(_), Some(_)) => panic!("Torrent `info` contains both `length` and `files`"),

            (None, None) => panic!("Torrent `info` contains neither `length` nor `files`"),

            (Some(length), None) => Self::Single {
                name,
                length: length as usize,
            },

            (None, Some(files)) => Self::Multi {
                name,
                files: files.into_iter().map(File::new).collect(),
            },
        }
    }
}

#[derive(Debug)]
struct File {
    path: Vec<String>,
    length: usize,
}

impl File {
    fn new(file_dict: Bencoded) -> Self {
        assert!(
            matches!(file_dict, Bencoded::Dict(_)),
            "Multi file entry is not a dictionary"
        );

        let length = get_int(&file_dict, "length")
            .map(|length| length as usize)
            .expect("Multi file entry does not have a `length` entry");

        let path = get_list(&file_dict, "path")
            .expect("Multi file entry does not have a `path` entry")
            .into_iter()
            .map(|sub_path| {
                if let Bencoded::Bstr(sub_path) = sub_path {
                    String::from_utf8(sub_path).expect("Multi file sub path is not valid UTF-8")
                } else {
                    panic!("Multi file sub path is not a byte string");
                }
            })
            .collect();

        Self { path, length }
    }
}

// Helper functions

fn get_bencoded_dict_value(dict: &Bencoded, key: &str) -> Option<Bencoded> {
    let key = Bencoded::Bstr(Vec::from(key.as_bytes()));

    if let Bencoded::Dict(pairs) = dict {
        pairs
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v.clone())
    } else {
        panic!("Not a dictionary");
    }
}

fn get_bstr(dict: &Bencoded, key: &str) -> Option<Vec<u8>> {
    match get_bencoded_dict_value(dict, key) {
        Some(Bencoded::Bstr(bstr)) => Some(bstr),
        _ => None,
    }
}

fn get_int(dict: &Bencoded, key: &str) -> Option<i64> {
    match get_bencoded_dict_value(dict, key) {
        Some(Bencoded::Int(int)) => Some(int),
        _ => None,
    }
}

fn get_list(dict: &Bencoded, key: &str) -> Option<Vec<Bencoded>> {
    match get_bencoded_dict_value(dict, key) {
        Some(Bencoded::List(list)) => Some(list),
        _ => None,
    }
}

fn get_dict(dict: &Bencoded, key: &str) -> Option<Bencoded> {
    let maybe_dict = get_bencoded_dict_value(dict, key);

    match maybe_dict {
        Some(Bencoded::Dict(_)) => maybe_dict,
        _ => None,
    }
}
