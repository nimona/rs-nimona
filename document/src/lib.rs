#![allow(unused_macros)]

use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum Field {
    Map(BTreeMap<String, Field>),
    String(String),
    Int64(i64),
    Uint64(u64),
    Bool(bool),
    Bytes(Vec<u8>),
    Array(Vec<Field>),
}

impl From<BTreeMap<String, Field>> for Field {
    fn from(value: BTreeMap<String, Field>) -> Self {
        Field::Map(value)
    }
}

impl Into<BTreeMap<String, Field>> for Field {
    fn into(self) -> BTreeMap<String, Field> {
        match self {
            Field::Map(map) => map,
            _ => panic!("Field is not a map"),
        }
    }
}

impl From<String> for Field {
    fn from(value: String) -> Self {
        Field::String(value)
    }
}

impl Into<String> for Field {
    fn into(self) -> String {
        match self {
            Field::String(s) => s,
            _ => panic!("Field is not a string"),
        }
    }
}

impl From<i64> for Field {
    fn from(value: i64) -> Self {
        Field::Int64(value)
    }
}

impl Into<i64> for Field {
    fn into(self) -> i64 {
        match self {
            Field::Int64(n) => n,
            _ => panic!("Field is not an int64"),
        }
    }
}

impl From<u64> for Field {
    fn from(value: u64) -> Self {
        Field::Uint64(value)
    }
}

impl Into<u64> for Field {
    fn into(self) -> u64 {
        match self {
            Field::Uint64(n) => n,
            _ => panic!("Field is not a uint64"),
        }
    }
}

impl From<bool> for Field {
    fn from(value: bool) -> Self {
        Field::Bool(value)
    }
}

impl Into<bool> for Field {
    fn into(self) -> bool {
        match self {
            Field::Bool(b) => b,
            _ => panic!("Field is not a bool"),
        }
    }
}

impl From<Vec<u8>> for Field {
    fn from(value: Vec<u8>) -> Self {
        Field::Bytes(value)
    }
}

impl Into<Vec<u8>> for Field {
    fn into(self) -> Vec<u8> {
        match self {
            Field::Bytes(b) => b,
            _ => panic!("Field is not bytes"),
        }
    }
}

impl From<Vec<Field>> for Field {
    fn from(value: Vec<Field>) -> Self {
        Field::Array(value)
    }
}

impl Into<Vec<Field>> for Field {
    fn into(self) -> Vec<Field> {
        match self {
            Field::Array(arr) => arr,
            _ => panic!("Field is not an array"),
        }
    }
}

impl From<Vec<BTreeMap<std::string::String, Field>>> for Field {
    fn from(value: Vec<BTreeMap<std::string::String, Field>>) -> Self {
        Field::Array(value.into_iter().map(Field::from).collect())
    }
}

impl Into<Vec<BTreeMap<std::string::String, Field>>> for Field {
    fn into(self) -> Vec<BTreeMap<std::string::String, Field>> {
        match self {
            Field::Array(arr) => arr.into_iter().map(Field::into).collect(),
            _ => panic!("Field is not an array"),
        }
    }
}

impl From<Vec<String>> for Field {
    fn from(value: Vec<String>) -> Self {
        Field::Array(value.into_iter().map(Field::from).collect())
    }
}

impl Into<Vec<String>> for Field {
    fn into(self) -> Vec<String> {
        match self {
            Field::Array(arr) => arr.into_iter().map(Field::into).collect(),
            _ => panic!("Field is not an array"),
        }
    }
}

impl From<Vec<i64>> for Field {
    fn from(value: Vec<i64>) -> Self {
        Field::Array(value.into_iter().map(Field::from).collect())
    }
}

impl Into<Vec<i64>> for Field {
    fn into(self) -> Vec<i64> {
        match self {
            Field::Array(arr) => arr.into_iter().map(Field::into).collect(),
            _ => panic!("Field is not an array"),
        }
    }
}

impl From<Vec<u64>> for Field {
    fn from(value: Vec<u64>) -> Self {
        Field::Array(value.into_iter().map(Field::from).collect())
    }
}

impl Into<Vec<u64>> for Field {
    fn into(self) -> Vec<u64> {
        match self {
            Field::Array(arr) => arr.into_iter().map(Field::into).collect(),
            _ => panic!("Field is not an array"),
        }
    }
}

impl From<Vec<bool>> for Field {
    fn from(value: Vec<bool>) -> Self {
        Field::Array(value.into_iter().map(Field::from).collect())
    }
}

impl Into<Vec<bool>> for Field {
    fn into(self) -> Vec<bool> {
        match self {
            Field::Array(arr) => arr.into_iter().map(Field::into).collect(),
            _ => panic!("Field is not an array"),
        }
    }
}

impl From<Vec<Vec<u8>>> for Field {
    fn from(value: Vec<Vec<u8>>) -> Self {
        Field::Array(value.into_iter().map(Field::from).collect())
    }
}

impl Into<Vec<Vec<u8>>> for Field {
    fn into(self) -> Vec<Vec<u8>> {
        match self {
            Field::Array(arr) => arr.into_iter().map(Field::into).collect(),
            _ => panic!("Field is not an array"),
        }
    }
}

pub struct Hash {
    bytes: Vec<u8>,
}

impl std::fmt::LowerHex for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.bytes.iter() {
            f.write_fmt(format_args!("{:02x}", byte))?;
        }
        Ok(())
    }
}

impl Field {
    fn key_prefix(&self) -> u8 {
        match self {
            Field::Map(_) => 'm' as u8,
            Field::String(_) => 's' as u8,
            Field::Int64(_) => 'i' as u8,
            Field::Uint64(_) => 'u' as u8,
            Field::Bool(_) => 'b' as u8,
            Field::Bytes(_) => 'x' as u8,
            Field::Array(_) => 'a' as u8,
        }
    }
    pub fn hash(&self) -> Hash {
        let mut hasher = Sha256::new();
        match self {
            Field::Map(doc) => {
                for (key, value) in doc {
                    // TODO: Does the key prefix need to be static/const?
                    hasher.update(['s' as u8]);
                    hasher.update(key.as_bytes());
                    hasher.update([value.key_prefix()]);
                    hasher.update(value.hash().bytes.as_slice());
                }
            }
            Field::String(s) => {
                hasher.update([self.key_prefix()]);
                hasher.update(s.as_bytes());
            }
            Field::Int64(n) => {
                hasher.update([self.key_prefix()]);
                hasher.update(&n.to_be_bytes());
            }
            Field::Uint64(n) => {
                hasher.update([self.key_prefix()]);
                hasher.update(&n.to_be_bytes());
            }
            Field::Bool(b) => {
                hasher.update([self.key_prefix()]);
                hasher.update([*b as u8]);
            }
            Field::Bytes(b) => {
                hasher.update([self.key_prefix()]);
                hasher.update(b.as_slice());
            }
            Field::Array(arr) => {
                hasher.update([self.key_prefix()]);
                for value in arr {
                    // TODO: Do individual array elements need a prefix?
                    hasher.update(value.hash().bytes.as_slice());
                }
            }
        }
        Hash {
            bytes: hasher.finalize().to_vec(),
        }
    }
}

