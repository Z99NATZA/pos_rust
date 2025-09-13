use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

use crate::app::result::AppResult;

pub fn password_hash(password_raw: String) -> AppResult<String> {
    let password = password_raw.as_bytes(); // Bad password; don't actually use!
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt)?.to_string();

    Ok(password_hash)
}