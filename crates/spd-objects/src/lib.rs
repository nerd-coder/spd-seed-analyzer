//! Shared game-object identities for the pinned SPD version.
//!
//! Every object the engine reports (items, enchantments, mobs, rooms, traps,
//! plants, blobs) is a numeric enum here. The wire format is the `u16` id;
//! `web/src/generated/game-ids.ts` is generated from these lists by
//! `bun run gen:ids`.

mod category;
mod lists;
pub mod typescript;

pub use category::ItemCategory;
pub use lists::*;

/// Defines a `#[repr(u16)]` object enum whose wire format is its id.
/// `Unknown` is id 0 and marks a class name with no variant yet.
#[macro_export]
macro_rules! object_enum {
    ($(#[$meta:meta])* $name:ident { $($variant:ident),* $(,)? }) => {
        $(#[$meta])*
        #[allow(clippy::upper_case_acronyms)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
        #[repr(u16)]
        pub enum $name {
            #[default]
            Unknown = 0,
            $($variant),*
        }

        impl $name {
            pub const ALL: &'static [$name] = &[$name::Unknown, $($name::$variant),*];

            /// Java simple class name, or `"Unknown"`.
            pub const fn class_name(self) -> &'static str {
                match self {
                    $name::Unknown => "Unknown",
                    $($name::$variant => stringify!($variant)),*
                }
            }

            pub fn from_class_name(class_name: &str) -> Option<Self> {
                match class_name {
                    $(stringify!($variant) => Some($name::$variant),)*
                    _ => None,
                }
            }

            /// `Unknown` when the class has no variant; tests sweep for it.
            pub fn from_class_name_lossy(class_name: &str) -> Self {
                Self::from_class_name(class_name).unwrap_or($name::Unknown)
            }

            pub const fn id(self) -> u16 {
                self as u16
            }

            pub fn from_id(id: u16) -> Option<Self> {
                Self::ALL.get(usize::from(id)).copied()
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self.class_name())
            }
        }

        impl PartialEq<str> for $name {
            fn eq(&self, other: &str) -> bool {
                self.class_name() == other
            }
        }

        impl PartialEq<&str> for $name {
            fn eq(&self, other: &&str) -> bool {
                self.class_name() == *other
            }
        }

        impl PartialEq<String> for $name {
            fn eq(&self, other: &String) -> bool {
                self.class_name() == other
            }
        }

        impl PartialEq<$name> for str {
            fn eq(&self, other: &$name) -> bool {
                self == other.class_name()
            }
        }

        impl PartialEq<$name> for &str {
            fn eq(&self, other: &$name) -> bool {
                *self == other.class_name()
            }
        }

        impl PartialEq<$name> for String {
            fn eq(&self, other: &$name) -> bool {
                self == other.class_name()
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_u16(self.id())
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let id = <u16 as ::serde::Deserialize>::deserialize(deserializer)?;
                $name::from_id(id).ok_or_else(|| {
                    ::serde::de::Error::custom(format!(
                        concat!("unknown ", stringify!($name), " id {}"),
                        id
                    ))
                })
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_round_trip_through_names_and_serde() {
        for &item in ItemId::ALL {
            assert_eq!(ItemId::from_id(item.id()), Some(item));
            if item != ItemId::Unknown {
                assert_eq!(ItemId::from_class_name(item.class_name()), Some(item));
            }
        }
        let json = serde_json::to_string(&ItemId::PotionOfHealing).unwrap();
        assert_eq!(json, ItemId::PotionOfHealing.id().to_string());
        assert_eq!(
            serde_json::from_str::<ItemId>(&json).unwrap(),
            ItemId::PotionOfHealing
        );
        assert!(serde_json::from_str::<ItemId>("65535").is_err());
    }

    #[test]
    fn generated_typescript_is_current() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../web/src/generated/game-ids.ts"
        );
        let committed = std::fs::read_to_string(path).unwrap_or_default();
        assert!(
            committed == typescript::render(),
            "{path} is stale; run `bun run gen:ids`"
        );
    }
}
