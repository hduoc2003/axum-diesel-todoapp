use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};


use crate::api::v1::types::{env::ENV, session::Session};

use super::env::get_env;

pub fn hash_password(password: &String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string()
}

pub fn verify_password(password: &String, password_hash: &String) -> bool {
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

pub fn verify_token(token: String) -> Session {
    let jwt_key = get_env(ENV::JWT_KEY);
    decode::<Session>(&token, &DecodingKey::from_secret(&jwt_key.as_bytes()), &Validation::default()).unwrap().claims
}

pub fn encrypt_session(session: &Session) -> String {
    let jwt_key = get_env(ENV::JWT_KEY);
    encode(&Header::default(), &session, &EncodingKey::from_secret(&jwt_key.as_bytes())).unwrap()
}
