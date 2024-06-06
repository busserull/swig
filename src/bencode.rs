#[derive(Debug)]
pub enum Bencoded {
    Bstr(String),
    Int(i64),
    List(Vec<Bencoded>),
    Dict(Vec<(Bencoded, Bencoded)>),
}

impl Bencoded {
    pub fn parse(encoded: &str) -> Option<Self> {
        Bencoded::do_parse(encoded).map(|(value, _)| value)
    }

    fn do_parse(encoded: &str) -> Option<(Self, &str)> {
        match encoded.chars().next() {
            Some('i') => {
                let end = encoded.find('e')?;

                encoded[1..end]
                    .parse::<i64>()
                    .ok()
                    .map(|int| (Self::Int(int), &encoded[end + 1..]))
            }

            Some('l') => {
                parse_bencoded_list(&encoded[1..]).map(|(list, rest)| (Self::List(list), rest))
            }

            Some('d') => {
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

            Some('0'..='9') => {
                let split_index = encoded.find(':')?;
                let size = *(&encoded[..split_index].parse::<usize>().ok()?);

                // The remainder of the string is less than the advertised size
                if encoded.len() - split_index - 1 < size {
                    return None;
                }

                Some((
                    Self::Bstr(String::from(
                        &encoded[split_index + 1..split_index + size + 1],
                    )),
                    &encoded[split_index + size + 2..],
                ))
            }

            _ => None,
        }
    }
}

fn parse_bencoded_list(mut encoded: &str) -> Option<(Vec<Bencoded>, &str)> {
    let mut list = Vec::new();

    while encoded.len() > 0 && encoded.chars().next().unwrap() != 'e' {
        let (item, rest) = Bencoded::do_parse(encoded)?;
        list.push(item);
        encoded = rest;
    }

    matches!(encoded.chars().next(), Some('e')).then_some((list, encoded))
}
