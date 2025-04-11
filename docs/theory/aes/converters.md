# AES Key Expansion Utilities

**Note:** All operations work with `4-byte` words.

## **1. `rot_word`**
Performs a 1-byte left circular shift on a 4-byte word.

```plaintext
Input:  [A, B, C, D]
Output: [B, C, D, A]
```

#### Using in AES:
Applied to every 4th word in KeyExpansion before SubWord.

---

## **2. `sub_word`**

Applies the S-Box substitution to each byte of a 4-byte word.

```plaintext
Input:    [0x00, 0x01, 0x02, 0x03]
Output:   [0x63, sbox[0x01], sbox[0x02], sbox[0x03]]
```

#### Using in AES:
Introduces non-linearity to prevent cryptographic attacks.

---

## **3. `xor_words`**
Computes a bitwise XOR between two 4-byte words.

```plaintext
[0xFF, 0x00, 0xAA, 0x55]  XOR
[0xAA, 0x55, 0xFF, 0x00]  =
[0x55, 0x55, 0x55, 0x55]
```

#### Using in AES
Core operation for generating new key schedule words.

---

## **How it works together:**

1. For each word `w[i]` (where `i = 8 to 59`):
    - When `i % 8` == 0:
    ```plaintext
    temp = sub_word(sbox, rot_word(w[i-1]))
    temp[0] ^= rcon[i//8]
    w[i] = xor_words(w[i-8], temp)         
    ```
    - Otherwise:
    ```paintext
    Simple XOR with previous word:
   w[i] = xor_words(w[i-8], w[i-1])
    ```
2. Final output: `60 words` -> `15 round keys` (each `16 bytes`)
