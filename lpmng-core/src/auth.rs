use biscuit_auth::{builder::*, error, Authorizer, Biscuit, KeyPair, PrivateKey};
use sha256::digest;

use crate::models::User;

pub fn build_token(role: String, private_key: PrivateKey) -> String {
    let root = KeyPair::from(private_key);

    let mut builder = Biscuit::builder(&root);

    builder
        .add_authority_fact(format!("role(\"{}\")", role).as_str())
        .unwrap();

    let biscuit = builder.build().unwrap();

    biscuit.to_base64().unwrap()
}

pub fn check_admin(auth_token: String, private_key: PrivateKey) -> bool {
    match Biscuit::from_base64(auth_token, |_| private_key.public()) {
        Ok(t) => {
            let mut auth = t.authorizer().unwrap();

            auth.add_check("check id role(\"admin\")");
            auth.allow();

            auth.authorize().is_ok()
        }
        Err(_) => false,
    }
}

pub fn hash(input: String) -> String {
    digest(input)
}

pub fn check_hash(input: String, h: String) -> bool {
    hash(input) == h
}
