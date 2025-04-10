# XOR Encryption Theory

## Core Principle
Bitwise XOR operation between data and key:

1. A ^ B = C 
2. C ^ B = A *// Decryption*

## Key Features
- **Symmetric**: Same key for encryption/decryption
- **Bit-level**: Works on raw bytes
- **Perfect secrecy**: When key is truly random and used once (OTP)

## Security Considerations
| Scenario          | Risk Level | Mitigation               |
|-------------------|------------|--------------------------|
| Reused key        | Critical   | Always generate new key  |
| Predictable key   | High       | Use cryptographic RNG    |
| Short key         | Medium     | Key length ≥ data length |

## UTF-8 Compatibility
- XOR operates on **raw bytes** after UTF-8 encoding
- Example for 'П' (UTF-8: `[208, 159]`):
  ```rust
  208 ^ 42  = 234  // Encrypted
  234 ^ 42  = 208  // Decrypted