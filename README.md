# SHAKE256 File Hasher (Post-Quantum Secure)

This Rust CLI tool hashes any file using the [SHAKE256](https://keccak.team/keccak.html) extendable-output function (XOF), part of the SHA-3 family. SHAKE256 is resistant to quantum attacks and is used in post-quantum cryptographic schemes such as SPHINCS+.

## 🔐 Why SHAKE256?

SHAKE256 is:
- **Post-quantum secure**: Resists both classical and quantum attacks.
- **Flexible**: You can specify any desired output length (256 bits, 512 bits, etc.).
- **Standardized**: Part of the NIST-approved SHA-3 suite.

## 🚀 Features

- Reads and hashes files of arbitrary size
- CLI interface using [clap](https://docs.rs/clap)
- Supports configurable hash output length (default: 64 bytes)
- Efficient and easy to use

## 📦 Installation

1. Clone the repo:
    `git clone https://github.com/yourusername/shake256_file_hasher.git
    cd shake256_file_hasher`

2. Build the project:
    `cargo build --release`

3. Usage
    `cargo run --release -- --input path/to/file --output-len 64`

## 🛠️ Dependencies
- sha3 
- clap
- hex

## 📜 License
This project is licensed under the MIT License.

## 🧠 Notes
SHAKE256 is ideal for applications where post-quantum security is a concern. It is used in various NIST post-quantum cryptographic standards like SPHINCS+ and is a strong choice for long-term secure hashing.
