use std::env;

mod bencode;

use bencode::Bencoded;

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded = Bencoded::parse(encoded_value);

        println!("{:?}", decoded);
    } else {
        println!("unknown command: {}", args[1])
    }
}
