# AES-256

## **AES Steps (Encrypt)**
| Step            | Description                                                                 | Mathematical Basis        |
|-----------------|-----------------------------------------------------------------------------|--------------------------|
| **SubBytes**    | Non-linear byte substitution via S-Box.                                     | GF(2⁸) inversion + Affine Transform |
| **ShiftRows**   | Cyclic left shift of rows in the State matrix.                              | Byte permutation         |
| **MixColumns**  | Linear transformation of each column using GF(2⁸) matrix multiplication.    | Polynomials in GF(2⁸)    |
| **AddRoundKey** | XOR between State and Round Key.                                           | Bitwise XOR              |

---

## **1. SubBytes**
- Each byte `a` in State is replaced by `SBox[a]`.
- **S-Box** = `GF256::inverse(a) + AffineTransform` (see [S-Box.md](../sbox.md)).

---

## **2. ShiftRows**
### **Encryption (ShiftRows)**
1. Cyclic **left** shift for each row:
    - **Row 0**: No shift.
    - **Row 1**: Shift by **1** → `[a, b, c, d]` → `[b, c, d, a]`.
    - **Row 2**: Shift by **2** → `[a, b, c, d]` → `[c, d, a, b]`.
    - **Row 3**: Shift by **3** → `[a, b, c, d]` → `[d, a, b, c]`.

2. **State Matrix Structure**
    - Bytes are stored **column-major**:
      ```plaintext
      | (0,0) | (0,1) | (0,2) | (0,3) |
      | (1,0) | (1,1) | (1,2) | (1,3) |
      | (2,0) | (2,1) | (2,2) | (2,3) |
      | (3,0) | (3,1) | (3,2) | (3,3) |
      ```
    - Where `(row, column)` indices match the AES spec.

---

## **3. MixColumns**

**Purpose**:
A linear transformation that shuffles the data within each column of the State matrix to provide cryptographic diffusion.

- Each column is multiplied by a fixed matrix in GF(2⁸):
```plaintext
    | 0x02 0x03 0x01 0x01 |   | a0 |   | b0 |
    | 0x01 0x02 0x03 0x01 | X | a1 | = | b1 |
    | 0x01 0x01 0x02 0x03 |   | a2 |   | b2 |
    | 0x03 0x01 0x01 0x02 |   | a3 |   | b3 |
```
- The `b` value can be obtained by following formula:
- `b = (a * const_1) + (a * const_2) + (a * const_3) + (a * const_4)`
  - `*` is a GP256 multiplication.
  - `+` is a `XOR`.
- **NOTE:** Values in the const matrix that equals 0x01 can be skipped in multiplying because `a * 0x01 = a`.

---

## **4. AddRoundKey _(Coming Soon!)_.**

---