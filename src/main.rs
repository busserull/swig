use serde_json;
use std::collections::HashMap;
use std::env;

mod bencode;

use bencode::Bencoded;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    if encoded_value.chars().next().unwrap().is_digit(10) {
        let colon_index = encoded_value.find(':').unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        return serde_json::Value::String(string.to_string());
    } else if encoded_value.chars().next().unwrap() == 'i' {
        let e_index = encoded_value.find('e').unwrap();
        let number_string = &encoded_value[..e_index];
        println!("Number string: {}", number_string);
        let number = number_string.parse::<i64>().unwrap();
        return number.into();
    } else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

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
