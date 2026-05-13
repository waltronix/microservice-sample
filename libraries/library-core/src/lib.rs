//! Pure domain ID primitives shared across services.
//!
//! Newtype wrappers around [`Uuid`] for type-safe identifiers. No business
//! logic, no external infrastructure dependencies.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            pub fn new(id: Uuid) -> Self {
                Self(id)
            }

            pub fn into_uuid(self) -> Uuid {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0, f)
            }
        }

        impl FromStr for $name {
            type Err = uuid::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Uuid::from_str(s).map(Self)
            }
        }
    };
}

define_id!(UserId);
define_id!(BookId);
define_id!(ReviewId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_id_roundtrips_through_string() {
        let id = UserId::new(Uuid::new_v4());
        let parsed: UserId = id.to_string().parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn book_id_roundtrips_through_string() {
        let id = BookId::new(Uuid::new_v4());
        let parsed: BookId = id.to_string().parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn review_id_roundtrips_through_string() {
        let id = ReviewId::new(Uuid::new_v4());
        let parsed: ReviewId = id.to_string().parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn into_uuid_returns_inner_value() {
        let uuid = Uuid::new_v4();
        let id = UserId::new(uuid);
        assert_eq!(id.into_uuid(), uuid);
    }

    #[test]
    fn from_str_rejects_invalid_uuid() {
        assert!("not-a-uuid".parse::<BookId>().is_err());
    }
}
