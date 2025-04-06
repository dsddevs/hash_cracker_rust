# SHA1 Cracker

## Overview

Welcome to `sha1_cracker` - a high-performance SHA1 hash password cracker. This tool is designed to efficiently crack SHA1 hashed passwords by comparing them against a list of potential plaintext passwords. Whether you're a security professional, a developer, or a tech enthusiast, `sha1_cracker` is an essential tool for your password cracking needs.

## Features

- **Fast and Efficient**: Utilizes the latest cryptographic libraries for swift password cracking.
- **Flexible Input**: Reads passwords from a specified file.
- **Robust Error Handling**: Provides detailed error messages for common issues.
- **Tested and Reliable**: Comes with unit tests to ensure reliability.

## Installation

To install `sha1_cracker`, you need to have [Rust](https://www.rust-lang.org/) installed. You can install Rust using [rustup](https://rustup.rs/). Once you have Rust installed, you can build the project using Cargo.

```sh
# Clone the repository
git clone https://github.com/dsddevs/hash_cracker_rust.git

# Navigate to the project directory
cd sha1_cracker

# Build the project
cargo build --release
```

## Usage

To use `sha1_cracker`, run the following command:

```sh
cargo run -- <file_path> <hash_value>
```

- `<file_path>`: The path to the file containing the list of potential plaintext passwords.
- `<hash_value>`: The SHA1 hash value you want to crack.

### Example

```sh
 cargo run -- wordlist.txt 7c6a61c68ef8b9b6b061b28c348bc1ed7921cb53
```

### Output

If the password is found, the tool will output:

```
Password found: <password>
```

If the password is not found, the tool will output:

```
Error: Password not found.
```

## Dependencies

This project relies on the following dependencies:

- [sha1](https://crates.io/crates/sha1): A Rust library for SHA1 hashing.
- [hex](https://crates.io/crates/hex): A Rust library for encoding and decoding hexadecimal values.

## Code Structure

- **cracker**: Contains the core password cracking logic.
    - `password_cracker.rs`: Defines the `PasswordCracker` trait.
    - `sha1_cracker.rs`: Implements the `PasswordCracker` trait for SHA1 hashing.
- **reader**: Contains the file reading logic.
    - `file_reader.rs`: Provides functionality to read files.

## Running Tests

To run the tests, use the following command:

```sh
cargo test
```

## Contributing

We welcome contributions to `sha1_cracker`. If you have a feature request, bug report, or proposal, feel free to open an issue or submit a pull request.

