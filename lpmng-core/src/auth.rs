use base64::encode;
use biscuit_auth::{builder::*, error, Authorizer, Biscuit, KeyPair};
use sha256::digest;

use crate::models::User;

pub fn build_token(role: String) -> String {
    let root = KeyPair::new();

    let mut builder = Biscuit::builder(&root);

    builder
        .add_authority_fact(format!("role(\"{}\")", role).as_str())
        .unwrap();

    let biscuit = builder.build().unwrap();

    encode(biscuit.to_vec().unwrap())
}

pub fn hash(input: String) -> String {
    digest(input)
}

pub fn check_hash(input: String, h: String) -> bool {
    hash(input) == h
}
