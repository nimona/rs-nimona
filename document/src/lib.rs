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

pub fn from_json(s: &str) -> Result<Field, serde_json::Error> {
    let value: serde_json::Value = serde_json::from_str(s)?;
    Ok(from_serde_value(value))
}

fn from_serde_value(value: serde_json::Value) -> Field {
    match value {
        serde_json::Value::Object(map) => {
            let mut btreemap = BTreeMap::new();
            for (key, value) in map {
                btreemap.insert(key, from_serde_value(value));
            }
            Field::Map(btreemap)
        }
        serde_json::Value::String(s) => Field::String(s),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Field::Int64(i)
            } else if let Some(u) = n.as_u64() {
                Field::Uint64(u)
            } else {
                panic!("Number is not an integer or unsigned integer")
            }
        }
        serde_json::Value::Bool(b) => Field::Bool(b),
        serde_json::Value::Array(a) => Field::Array(a.into_iter().map(from_serde_value).collect()),
        serde_json::Value::Null => panic!("Null values are not supported"),
    }
}

pub fn to_json(field: &Field) -> Result<String, serde_json::Error> {
    let value = to_serde_value(field);
    serde_json::to_string(&value)
}

fn to_serde_value(field: &Field) -> serde_json::Value {
    match field {
        Field::Map(map) => {
            let mut object = serde_json::Map::new();
            for (key, value) in map {
                object.insert(key.clone(), to_serde_value(value));
            }
            serde_json::Value::Object(object)
        }
        Field::String(s) => serde_json::Value::String(s.clone()),
        Field::Int64(i) => serde_json::Value::Number(serde_json::Number::from(*i)),
        Field::Uint64(u) => serde_json::Value::Number(serde_json::Number::from(*u)),
        Field::Bool(b) => serde_json::Value::Bool(*b),
        Field::Bytes(bytes) => serde_json::Value::String(base64::encode(bytes)),
        Field::Array(a) => {
            serde_json::Value::Array(a.iter().map(to_serde_value).collect::<Vec<_>>())
        }
    }
}

// test from_json
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_from_json() {
        let f_document = from_json(r#"{"a": 1, "b": "Hello"}"#).unwrap();
        println!("{:?}", f_document);
        println!("{:x}", f_document.hash());

        // Encode document to JSON
        let f_document_json_result = to_json(&f_document);
        match f_document_json_result {
            Ok(json) => println!("{}", json),
            Err(e) => println!("Error: {}", e),
        }

        // Hash document
        println!("{:x}", f_document.hash());
    }
}
