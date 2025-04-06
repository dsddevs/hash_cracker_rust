use std::error::Error;
use std::io::BufRead;

pub trait PasswordCracker {
    fn crack_password(reader: impl BufRead, target_hash: &str) -> Result<(), Box<dyn Error>>;
}