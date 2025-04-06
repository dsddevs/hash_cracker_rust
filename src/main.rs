use process::exit;
use std::{env, process};
use std::error::Error;
use crate::cracker::password_cracker::PasswordCracker;
use crate::cracker::sha1_cracker::Sha1Cracker;
use crate::reader::file_reader::FileReader;

mod cracker;
mod test;
mod reader;

fn main() -> Result<(), Box<dyn Error>> {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: cargo run -- <file_path> <hash_value>");
        exit(1);
    };

    let file_path = &args[1];
    let target_hash = &args[2];

    if target_hash.len() != 40 {
        return Err("Invalid SHA1 hash length. Must be 40 hex characters.".into());
    }

    let reader = FileReader::from_path(file_path)?;
    Sha1Cracker::crack_password(reader, target_hash)?;
    Ok(())


}

