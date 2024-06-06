use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub enum Bencoded {
    Bstr(String),
    Int(i64),
    List(Vec<Bencoded>),
    Dict(HashMap<Bencoded, Bencoded>),
}

impl Bencoded {
    pub fn parse(encoded: &str) -> Option<Self> {
        Bencoded::do_parse(encoded)
    }

    fn do_parse(encoded: &str) -> Option<Self> {
        match encoded.chars().next() {
            Some('i') => {
                let end = encoded.find('e')?;
                encoded[1..end].parse::<i64>().ok().map(Self::Int)
            }

            Some('l') => None,

            Some('d') => None,

            Some('0'..='9') => {
                let split_index = encoded.find(':')?;
                let size = *(&encoded[..split_index].parse::<usize>().ok()?);

                // The remainder of the string is less than the advertised size
                if encoded.len() - split_index - 1 < size {
                    return None;
                }

                Some(Self::Bstr(String::from(
                    &encoded[split_index + 1..split_index + size + 1],
                )))
            }

            _ => None,
        }
    }
}
