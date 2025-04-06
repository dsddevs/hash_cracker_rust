use crate::cracker::password_cracker::PasswordCracker;
use sha1::{Digest, Sha1};
use std::error::Error;
use std::io::BufRead;

pub struct Sha1Cracker;

impl PasswordCracker for Sha1Cracker {
    fn crack_password(reader: impl BufRead, target_hash: &str) -> Result<(), Box<dyn Error>> {
        for line in reader.lines() {
            let line = line?;
            let password = line.trim();
            let sha1_hash = Sha1::digest(password.as_bytes());
            let sha1_hash_hex = hex::encode(sha1_hash);
            if sha1_hash_hex == target_hash {
                println!("Password found: {password}");
                return Ok(())
            }
        }
        eprintln!("Error: Password not found.");
        Ok(())
    }
}
