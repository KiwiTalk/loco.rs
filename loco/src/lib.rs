pub mod api;
pub mod codec;
pub mod config;
pub mod crypto;
mod error;
pub mod types;

pub use error::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(untagged)]
pub enum Nonexhaustive<T> {
    Known(T),
    Unknown(i32),
}

impl<T> Nonexhaustive<T> {
    pub fn is_known(&self) -> bool {
        matches!(self, Known(_))
    }
}

pub use Nonexhaustive::*;

#[cfg(test)]
mod test {
    use super::*;
    use serde_repr::{Deserialize_repr, Serialize_repr};
    use serde_test::{assert_tokens, Token};

    #[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq, Clone)]
    #[repr(i32)]
    enum MyEnum {
        TheOnlyOne = 1,
    }

    #[test]
    fn nonexhaustive_known_ser_de() {
        let known = Known(MyEnum::TheOnlyOne);
        assert_tokens(&known, &[Token::I32(1)]);
    }

    #[test]
    fn nonexhaustive_unknown_ser_de() {
        let unknown: Nonexhaustive<MyEnum> = Unknown(13);
        assert_tokens(&unknown, &[Token::I32(13)]);
    }
}
