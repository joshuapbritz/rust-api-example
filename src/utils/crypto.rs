pub mod pwdhash {
    use crate::errors::ServiceError;
    use argon2::{
        Argon2,
        password_hash::{
            PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
        },
    };

    pub fn hash_password(password: &str) -> Result<String, ServiceError> {
        let salt = create_salt();

        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| ServiceError::BadRequest)?
            .to_string();

        Ok(hash)
    }

    pub fn verify(hash: &str, password: &str) -> Result<(), ServiceError> {
        let parsed_hash = PasswordHash::new(&hash).map_err(|_| ServiceError::Unauthorized)?;

        let result = Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        match result {
            true => Ok(()),
            false => Err(ServiceError::Unauthorized),
        }
    }

    pub fn create_salt() -> SaltString {
        SaltString::generate(&mut OsRng)
    }
}
