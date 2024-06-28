use std::borrow::Cow;

use nimiq_database_value::{AsDatabaseBytes, FromDatabaseBytes};

use crate::traits::{DupTableValue, Key, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedValue<I: Key, V: Value> {
    pub index: I,
    pub value: V,
}

impl<I: Key, V: Value> IndexedValue<I, V> {
    pub fn new(index: I, value: V) -> Self {
        Self { index, value }
    }
}

impl<I: Key, V: Value> DupTableValue for IndexedValue<I, V> {
    type SubKey = I;

    type Value = V;

    fn subkey(&self) -> &Self::SubKey {
        &self.index
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

impl<I: Key, V: Value> AsDatabaseBytes for IndexedValue<I, V> {
    fn as_key_bytes(&self) -> Cow<[u8]> {
        let bytes = [
            &self.index.as_value_bytes()[..],
            &self.value.as_value_bytes()[..],
        ]
        .concat();
        Cow::Owned(bytes)
    }

    const FIXED_SIZE: Option<usize> = match (I::FIXED_SIZE, V::FIXED_SIZE) {
        (Some(index_len), Some(value_len)) => Some(index_len + value_len),
        (Some(_), None) => None,
        (None, _) => panic!("Index must have a fixed size"),
    };
}

impl<I: Key, V: Value> FromDatabaseBytes for IndexedValue<I, V> {
    fn from_key_bytes(bytes: &[u8]) -> Self
    where
        Self: Sized,
    {
        let index_size = I::FIXED_SIZE.expect("Index must have a fixed size");
        IndexedValue {
            index: I::from_value_bytes(&bytes[..index_size]),
            value: V::from_value_bytes(&bytes[index_size..]),
        }
    }
}
