use argon2::{self, Config};
use chrono::Utc;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub email: String,
}

pub fn create_hash(password: String) -> String {
    let password = password.as_bytes();
    let salt = env::var("SALT").expect("SALT must be set in .env file");
    let bsalt = salt.as_bytes();
    let config = Config::default();
    let hash = argon2::hash_encoded(&password, &bsalt, &config).unwrap();

    hash
}

pub fn compare_hash(hashed: String, password: String) -> bool {
    let password = password.as_bytes();
    argon2::verify_encoded(&hashed, &password).unwrap()
}

pub fn create_jwt(email: String) -> String {
    let expire = Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .unwrap()
        .timestamp();

    let claims = Claims {
        exp: expire as usize,
        email: email,
    };

    let header = Header::new(Algorithm::HS256);
    let secret = env::var("SECRET_KEY").expect("SECRET must be set in .env file");

    let bsecret = secret.as_bytes();
    let token = encode(&header, &claims, &EncodingKey::from_secret(&bsecret)).unwrap();

    token
}

pub fn decode_jwt(token: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let result = Ok(decode::<Claims>(
        &token,
        &DecodingKey::from_secret(
            env::var("SECRET_KEY")
                .expect("SECRET must be set in .env file")
                .as_bytes(),
        ),
        &Validation::new(Algorithm::HS256),
    )?);

    result
}
