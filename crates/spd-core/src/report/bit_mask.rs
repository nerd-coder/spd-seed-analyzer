//! Per-cell boolean masks serialized as `0`/`1` numbers: about a third of the
//! JSON weight of `true`/`false`, and still truthy/falsy for JS consumers.

use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserializer, Serializer};

pub fn serialize<S: Serializer>(mask: &[bool], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(mask.iter().map(|&cell| cell as u8))
}

/// Accepts both the `0`/`1` wire form and plain booleans.
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<bool>, D::Error> {
    struct MaskVisitor;

    impl<'de> Visitor<'de> for MaskVisitor {
        type Value = Vec<bool>;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a sequence of 0/1 or booleans")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Vec<bool>, A::Error> {
            let mut mask = Vec::with_capacity(seq.size_hint().unwrap_or(0));
            while let Some(cell) = seq.next_element::<Cell>()? {
                mask.push(cell.0);
            }
            Ok(mask)
        }
    }

    deserializer.deserialize_seq(MaskVisitor)
}

struct Cell(bool);

impl<'de> serde::Deserialize<'de> for Cell {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct CellVisitor;

        impl Visitor<'_> for CellVisitor {
            type Value = Cell;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("0, 1, or a boolean")
            }

            fn visit_bool<E: de::Error>(self, value: bool) -> Result<Cell, E> {
                Ok(Cell(value))
            }

            fn visit_u64<E: de::Error>(self, value: u64) -> Result<Cell, E> {
                match value {
                    0 | 1 => Ok(Cell(value == 1)),
                    _ => Err(E::invalid_value(de::Unexpected::Unsigned(value), &self)),
                }
            }

            fn visit_i64<E: de::Error>(self, value: i64) -> Result<Cell, E> {
                match value {
                    0 | 1 => Ok(Cell(value == 1)),
                    _ => Err(E::invalid_value(de::Unexpected::Signed(value), &self)),
                }
            }
        }

        deserializer.deserialize_any(CellVisitor)
    }
}

#[cfg(test)]
mod tests {
    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct Wrapper(#[serde(with = "super")] Vec<bool>);

    #[test]
    fn round_trips_and_accepts_booleans() {
        let mask = Wrapper(vec![true, false, false, true]);
        let json = serde_json::to_string(&mask).unwrap();
        assert_eq!(json, "[1,0,0,1]");
        assert_eq!(serde_json::from_str::<Wrapper>(&json).unwrap(), mask);
        assert_eq!(
            serde_json::from_str::<Wrapper>("[true,false,false,true]").unwrap(),
            mask
        );
        assert!(serde_json::from_str::<Wrapper>("[2]").is_err());
    }
}
