//! Per-cell byte arrays serialized as a standard (padded) base64 string: about
//! 1.3 JSON bytes per cell instead of up to 3 for a number array.

use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserializer, Serializer};

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&encode(bytes))
}

/// Accepts both the base64 wire form and a plain number array.
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    struct BytesVisitor;

    impl<'de> Visitor<'de> for BytesVisitor {
        type Value = Vec<u8>;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a base64 string or a sequence of bytes")
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<Vec<u8>, E> {
            decode(value).ok_or_else(|| E::invalid_value(de::Unexpected::Str(value), &self))
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Vec<u8>, A::Error> {
            let mut bytes = Vec::with_capacity(seq.size_hint().unwrap_or(0));
            while let Some(byte) = seq.next_element::<u8>()? {
                bytes.push(byte);
            }
            Ok(bytes)
        }
    }

    deserializer.deserialize_any(BytesVisitor)
}

fn encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = (b[0] as u32) << 16 | (b[1] as u32) << 8 | b[2] as u32;
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(ALPHABET[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

fn decode(text: &str) -> Option<Vec<u8>> {
    let text = text.as_bytes();
    if !text.len().is_multiple_of(4) {
        return None;
    }
    let mut out = Vec::with_capacity(text.len() / 4 * 3);
    for chunk in text.chunks(4) {
        let pad = chunk.iter().rev().take_while(|&&c| c == b'=').count();
        if pad > 2 {
            return None;
        }
        let mut n = 0u32;
        for &c in &chunk[..4 - pad] {
            n = n << 6 | ALPHABET.iter().position(|&a| a == c)? as u32;
        }
        n <<= 6 * pad as u32;
        out.extend_from_slice(&n.to_be_bytes()[1..4 - pad]);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct Wrapper(#[serde(with = "super")] Vec<u8>);

    #[test]
    fn round_trips_every_padding_length() {
        for len in 0..8 {
            let bytes = Wrapper((0..len).map(|i| (i * 37 + 99) as u8).collect());
            let json = serde_json::to_string(&bytes).unwrap();
            assert_eq!(serde_json::from_str::<Wrapper>(&json).unwrap(), bytes);
        }
    }

    #[test]
    fn matches_standard_base64_and_accepts_arrays() {
        let json = serde_json::to_string(&Wrapper(b"spd-v4".to_vec())).unwrap();
        assert_eq!(json, "\"c3BkLXY0\"");
        let json = serde_json::to_string(&Wrapper(vec![0, 99, 255, 7])).unwrap();
        assert_eq!(json, "\"AGP/Bw==\"");
        assert_eq!(
            serde_json::from_str::<Wrapper>("[0,99,255,7]").unwrap(),
            Wrapper(vec![0, 99, 255, 7])
        );
        assert!(serde_json::from_str::<Wrapper>("\"AGP\"").is_err());
        assert!(serde_json::from_str::<Wrapper>("\"A*P/\"").is_err());
    }
}
