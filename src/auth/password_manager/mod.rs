use bcrypt::{DEFAULT_COST, hash, verify};

pub struct PasswordManager;

impl PasswordManager {
    pub fn hash_password(password: String) -> Result<String, bcrypt::BcryptError> {
        hash(password, DEFAULT_COST)
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
        verify(password, hash)
    }
}
