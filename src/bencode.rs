use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bencoded {
    Bstr(Vec<u8>),
    Int(i64),
    List(Vec<Bencoded>),
    Dict(Vec<(Bencoded, Bencoded)>),
}

impl Bencoded {
    pub fn parse(encoded: &[u8]) -> Option<Self> {
        Bencoded::do_parse(encoded).map(|(value, _)| value)
    }

    fn do_parse(encoded: &[u8]) -> Option<(Self, &[u8])> {
        match encoded.iter().next() {
            Some(b'i') => {
                let end = encoded.iter().position(|&byte| byte == b'e')?;

                String::from_utf8(Vec::from(&encoded[1..end]))
                    .ok()
                    .map(|string| string.parse::<i64>().ok())
                    .flatten()
                    .map(|int| (Self::Int(int), &encoded[end + 1..]))
            }

            Some(b'l') => {
                parse_bencoded_list(&encoded[1..]).map(|(list, rest)| (Self::List(list), rest))
            }

            Some(b'd') => {
                let (list, rest) = parse_bencoded_list(&encoded[1..])?;

                if list.len() % 2 != 0 {
                    return None;
                }

                let mut key_values = Vec::with_capacity(list.len() / 2);

                let mut list = list.into_iter();

                while list.len() != 0 {
                    let k = list.next().unwrap();
                    let v = list.next().unwrap();

                    key_values.push((k, v));
                }

                Some((Self::Dict(key_values), rest))
            }

            Some(b'0'..=b'9') => {
                let split_index = encoded.iter().position(|&byte| byte == b':')?;

                let size = String::from_utf8(Vec::from(&encoded[..split_index]))
                    .ok()
                    .map(|string| string.parse::<usize>().ok())
                    .flatten()?;

                if encoded.len() - split_index - 1 < size {
                    return None;
                }

                Some((
                    Self::Bstr(Vec::from(&encoded[split_index + 1..split_index + size + 1])),
                    &encoded[split_index + size + 1..],
                ))
            }

            _ => None,
        }
    }
}

impl fmt::Display for Bencoded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bstr(bstr) => write!(f, "\"{}\"", String::from_utf8_lossy(bstr)),

            Self::Int(int) => write!(f, "{}", int),

            Self::List(list) => {
                write!(f, "[")?;

                for item in &list[..list.len() - 1] {
                    write!(f, "{}, ", item)?;
                }

                if let Some(item) = list.last() {
                    write!(f, "{}", item)?;
                }

                write!(f, "]")
            }

            Self::Dict(pairs) => {
                write!(f, "{{")?;

                for (k, v) in &pairs[..pairs.len() - 1] {
                    write!(f, "{}: {}, ", k, v)?;
                }

                if let Some((k, v)) = pairs.last() {
                    write!(f, "{}: {}", k, v)?;
                }

                write!(f, "}}")
            }
        }
    }
}

fn parse_bencoded_list(mut encoded: &[u8]) -> Option<(Vec<Bencoded>, &[u8])> {
    let mut list = Vec::new();

    while encoded.len() > 0 && *encoded.iter().next().unwrap() != b'e' {
        let (item, rest) = Bencoded::do_parse(encoded)?;
        list.push(item);
        encoded = rest;
    }

    matches!(encoded.iter().next().as_deref(), Some(b'e')).then_some((list, &encoded[1..]))
}
