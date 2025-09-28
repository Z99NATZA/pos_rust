use std::env;

use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[test]
pub fn test_claims() {
    if cfg!(debug_assertions) {
        dotenvy::dotenv().ok();
    }

    let jwt_secret = env::var("JWT_SECRET").unwrap();

    let my_claims = Claims {
        sub: "3a4eb578-91cd-4354-bdf0-540bed747f4e".to_string(),
        exp: 10000
    };

    let token = encode(
        &Header::default(), 
        &my_claims, 
        &EncodingKey::from_secret(jwt_secret.as_ref())
    ).unwrap();

    println!("{token}");
}