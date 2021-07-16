use std::fmt::Display;

#[derive(Debug, Default, PartialEq)]
pub struct BetterType {
    key: String,
    value: String,
}

impl BetterType {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

impl Display for BetterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BetterType {{ key: {}, value: {} }}",
            self.key, self.value
        )
    }
}
