#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use sha1::Digest;
    use crate::cracker::password_cracker::PasswordCracker;
    use crate::cracker::sha1_cracker::Sha1Cracker;

    #[test]
    fn test_crack_password_finds_match() {
        let passwords = "password123\ndsd\n123456";
        let target_hash = hex::encode(sha1::Sha1::digest("dsd".as_bytes()));
        let cursor_reader = Cursor::new(passwords);

        let result = Sha1Cracker::crack_password(cursor_reader, &target_hash);
        assert!(result.is_ok());
    }
}