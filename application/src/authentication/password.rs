use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub fn hash_password(password: &str) -> Result<String, String> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| format!("Error hashing password: {:?}", err))?;
    Ok(password_hash.to_string())
}

pub fn verify_password(inc_password: &str, password: &str) -> Result<bool, String> {
    let argon2 = Argon2::default();
    let parsed_password_hash = PasswordHash::new(password)
        .map_err(|err| format!("Error parsing password hash: {:?}", err))?;
    Ok(argon2
        .verify_password(inc_password.as_bytes(), &parsed_password_hash)
        .is_ok())
}
