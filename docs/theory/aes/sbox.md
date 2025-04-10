# S-Box creating algorithm

## **1. Finding the Multiplicative Inverse**

Foreach byte of `a` finds an inverse value `a⁻¹`. 
1. If `a = 0`:
    - **Special case:** `0x00` has no inverse.
    - **In S-Box:** Directly use `0x63` (part of affine transform constant).
2. If `a ≠ 0` we have a special algorithm:
    - `a * a⁻¹ ≡ 1 mod 0x11B`.
    - Algorithm steps:
      1. **Note:** `a⁻¹ = a²⁵⁴`, because `a255 = 1`.
      2. **Initialize:**
         - `result = 1`
         - `a_pow = a`
         - `n = 254` (`0b11111110`), where `n` is degree
      3. **Loop while n > 0:**
         - **If `LSB` (the least significant bit) is 1** 
           - Multiply `result` by `a_pow` in GF(2⁸) (`result = gf256_mul(result, a_pow)`).
         - **Square a pow** (`a_pow = gf256_mul(a_pow, a_pow)`).
         - **Right-shift `n`** (`n >>= 1`).
      4. **Result:** result contains `a⁻¹`.


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

    pub fn inverse(a: u8) -> u8 {
        if a == 0 { 0 } else { Self::pow(a, 254) }
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