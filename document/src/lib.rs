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
