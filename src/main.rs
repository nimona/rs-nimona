use std::collections::BTreeMap;

use document_derive::{FromDocument, ToDocument};
use rand_core::OsRng;
use x25519_dalek::{EphemeralSecret, PublicKey};

#[derive(Debug, Default, Clone, ToDocument, FromDocument, PartialEq)]
#[my_trait(answer = 9)]
struct Test {
    map: BTreeMap<String, document::Field>,
    string: String,
    i64: i64,
    u64: u64,
    bool: bool,
    bytes: Vec<u8>,
    map_array: Vec<BTreeMap<String, document::Field>>,
    string_array: Vec<String>,
    i64_array: Vec<i64>,
    u64_array: Vec<u64>,
    bool_array: Vec<bool>,
    bytes_array: Vec<Vec<u8>>,
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Metadata {
    owner: String,
    permissions: Permissions,
    created: u64,
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Permissions {
    paths: Vec<String>,
    operations: Vec<String>,
    conditions: Vec<String>,
}

trait ToDocument {
    fn to_document(self) -> document::Field;
}

trait FromDocument {
    fn from_document(doc: document::Field) -> Result<Self, String>
    where
        Self: Sized;
}

fn main() {
    // Decode document from JSON
    let f_document = document::from_json(r#"{"a": 1, "b": "Hello"}"#).unwrap();
    println!("{:?}", f_document);
    println!("{:x}", f_document.hash());

    // Encode document to JSON
    let f_document_json_result = document::to_json(&f_document);
    match f_document_json_result {
        Ok(json) => println!("{}", json),
        Err(e) => println!("Error: {}", e),
    }

    // Hash document
    println!("{:x}", f_document.hash());

    // Create alice's keys
    let alice_secret = EphemeralSecret::new(OsRng);
    let alice_public = PublicKey::from(&alice_secret);

    // Create bob's keys
    let bob_secret = EphemeralSecret::new(OsRng);
    let bob_public = PublicKey::from(&bob_secret);

    // Calculate shared secrets
    let alice_shared_secret = alice_secret.diffie_hellman(&bob_public);
    let bob_shared_secret = bob_secret.diffie_hellman(&alice_public);
    let alice_shared_secret_hex = hex::encode(alice_shared_secret.as_bytes());
    println!("Alice's shared secret: {}", alice_shared_secret_hex);
    assert_eq!(alice_shared_secret.as_bytes(), bob_shared_secret.as_bytes());

    // Document from struct
    // let metadata = document::Metadata {
    //     owner: "John Doe".to_owned(),
    //     permissions: document::Permissions {
    //         paths: vec!["/path/to/file".to_owned(), "/path/to/other/file".to_owned()],
    //         operations: vec!["read".to_owned(), "write".to_owned()],
    //         conditions: vec!["if owner".to_owned(), "if age > 18".to_owned()],
    //     },
    //     created: 1623576001,
    // };

    // Construct Test struct
    let test_struct = Test {
        map: BTreeMap::new(),
        string: "Hello".to_owned(),
        i64: 42,
        u64: 42,
        bool: true,
        bytes: vec![1, 2, 3, 4],
        map_array: vec![BTreeMap::new()],
        string_array: vec!["Hello".to_owned()],
        i64_array: vec![42],
        u64_array: vec![42],
        bool_array: vec![true],
        bytes_array: vec![vec![1, 2, 3, 4]],
    };

    // Convert Test struct to document
    let test_doc = test_struct.to_document();
    println!("{:?}", test_doc);

    // Convert document to Test struct
    let test_doc_second = Test::from_document(test_doc);
    println!("{:?}", test_doc_second);
    // assert_eq!(test_struct.clone(), test_doc_second.unwrap());
}
