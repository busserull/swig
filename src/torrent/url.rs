use super::sha::Sha1;

pub struct Url {
    base: String,
    params: Vec<(String, String)>,
}

impl Url {
    pub fn new(base: &str) -> Self {
        Self {
            base: String::from(base),
            params: Vec::new(),
        }
    }

    pub fn with_param<P: UrlParamable>(mut self, name: &str, param: P) -> Self {
        self.params.push((String::from(name), param.as_url_param()));
        self
    }
}

impl From<Url> for String {
    fn from(url: Url) -> Self {
        let mut string = url.base;

        if url.params.is_empty() {
            return string;
        }

        string.push('?');

        for (key, value) in &url.params[..url.params.len() - 1] {
            string.push_str(&key);
            string.push('=');
            string.push_str(&value);
            string.push('&');
        }

        if let Some((key, value)) = url.params.last() {
            string.push_str(&key);
            string.push('=');
            string.push_str(&value);
        }

        string
    }
}

pub trait UrlParamable {
    fn as_url_param(self) -> String;
}

impl UrlParamable for &str {
    fn as_url_param(self) -> String {
        String::from(self)
    }
}

impl UrlParamable for Sha1 {
    fn as_url_param(self) -> String {
        self.as_ref().as_url_param()
    }
}

impl UrlParamable for &[u8] {
    fn as_url_param(self) -> String {
        let mut buffer = String::with_capacity(3 * self.len());

        for (i, nibble) in hex::encode(self).chars().enumerate() {
            if i % 2 == 0 {
                buffer.push_str("%");
            }

            buffer.push(nibble.to_ascii_uppercase());
        }

        buffer
    }
}

impl UrlParamable for usize {
    fn as_url_param(self) -> String {
        format!("{}", self)
    }
}
