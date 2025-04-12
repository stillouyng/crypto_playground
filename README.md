# crypto_playground

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

**A sandbox for encryption experiments in Rust**  
Explore algorithms, test implementations, and break ciphers - all in one playground!


### Roadmap
| Method          | Encode status | Decode status | Documentation                                                                                            |
|-----------------|---------------|---------------|----------------------------------------------------------------------------------------------------------|
| `Utf8->binary`  | ✅             | ❌             | [UTF8.md](docs/theory/utf8.md)                                                                           |
| `XOR`           | ✅             | ❌             | [XOR.md](docs/theory/xor.md)                                                                             |
| `GF256`         | ✅             | ❌             | [GF256.md](docs/theory/gf256.md)                                                                         |
| `S-Box`         | ✅             | ❌             | [SBox.md](docs/theory/sbox.md)                                                                           |
| `Key Expansion` | ✅             | ❌             | [KeyExpansion.md](docs/theory/aes/key_expansion.md)<br><br>[Utilities.md](docs/theory/aes/converters.md) |
| `AES-256`       | ✅             | ❌             | [AES.md](docs/theory/aes/aes.md)                                                                         |