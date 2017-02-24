//! Wrap url::Url with serde support

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, Visitor};
use std::fmt;
use std::ops::Deref;

/// Ser-de wrapper for url::Url struct
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Url(::url::Url);

impl Deref for Url {
    type Target = ::url::Url;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<::url::Url> for Url {
    fn from(url: ::url::Url) -> Url {
        Url(url)
    }
}

impl Url {
    pub fn parse(input: &str) -> Result<Self, ::url::ParseError> {
        ::url::Url::parse(&input).map(|u| u.into())
    }
}

impl Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", self.0);
        serializer.serialize_str(&s)
    }
}

struct UrlVisitor;

impl<'de> Visitor<'de> for UrlVisitor {
    type Value = Url;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an url")
    }

    fn visit_str<E>(self, value: &str) -> Result<Url, E>
    where
        E: de::Error,
    {
        Url::parse(value).map_err(E::custom)
    }

    fn visit_string<E>(self, v: String) -> Result<Url, E>
    where
        E: de::Error,
    {
        Url::parse(&v).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(UrlVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::Url;

    #[test]
    fn ser_de() {
        let url = Url::parse("http://linhehuo.com/image.png").unwrap();
        let json = ::serde_json::to_string(&url).unwrap();
        println!("{}", json);
        assert_eq!(json, r#""http://linhehuo.com/image.png""#);
        let new = ::serde_json::from_str(&json).unwrap();
        assert_eq!(url, new);
    }
}
