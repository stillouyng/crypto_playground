# S-Box (Substitution Box) for AES

## Overview
The S-Box is a **non-linear substitution table** used in AES (Rijndael cipher) to provide confusion.
Each byte in the AES state is replaced via this table during encryption.

## How It Works

## **1. Finding the Multiplicative Inverse**

- For input byte `a`, compute its multiplicative inverse `a⁻¹` in GF(256).
- Except `a=0`, which maps to `0`.

**Rust implementation**
```rust
impl GF256 {
    pub fn pow(a: u8, power: u8) -> u8 {
        let mut result = 1;
        let mut a_pow = a;
        let mut n = power;
        
        while n > 0 {
            if n & 1 == 1 {
                result = Self::mul(result, a_pow);
            }
            a_pow = Self::mul(a_pow, a_pow);
            n >>= 1;
        }
        result
    }
}
```


## **2. Affine transformation.**
*Applied after computing the multiplicative inverse `a⁻¹` to:*
- **Introduce non-linearity.** Resist linear attacks.
- **Eliminate fixed points.** S-box[a] ≠ a.
- **Improve bit diffusion**. Each bit of output depends on several bits of input.

#### Algorithm steps:
1. **Initialize.**
    - `b = a⁻¹`.
    - `const 0x63`.
2. **Apply transformation:`**
   1. Each bit in the input byte `b` is transformed using the following formula:
   2. `b'_i = b_i ⊕ b_{(i+4)%8} ⊕ b_{(i+5)%8} ⊕ b_{(i+6)%8} ⊕ b_{(i+7)%8} ⊕ c_i`.
   3. Here:
      - `b_i` is the i-th bit of the input byte,
      - `c_i` is the i-th bit of the constant `0x63`,
      - all operations are performed over GF256, which means using bitwise XOR
   4. For each index `i` (from 0 to 7), the i-th output bit `b'_i` is calculated by taking:
      - the current bit `b_i`,
      - the bits rotated by 4, 5, 6, and 7 positions (modulo 8),
      - and finally XORed with the corresponding bit of the constant.
   5. The resulting bits `b'_0` through `b'_7` are then combined into the final byte using:
      - ```rust
            affined_response |= result_bit << i;
        ```
3. **Return `result`:**
    - `S-Box[a⁻¹] = result`.

**Rust implementation**
```rust
fn affine_transform_bitwise(b: u8) -> u8 {
   let mut result = 0u8;
   let constant = 0x63;

   for i in 0..8 {
      let bit = ((b >> i) & 1) ^
              ((b >> ((i + 4) % 8)) & 1) ^
              ((b >> ((i + 5) % 8)) & 1) ^
              ((b >> ((i + 6) % 8)) & 1) ^
              ((b >> ((i + 7) % 8)) & 1) ^
              ((constant >> i) & 1);

      result |= bit << i;
   }

   result
}
```

[FIPS-197, Section 5.1.1](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf).