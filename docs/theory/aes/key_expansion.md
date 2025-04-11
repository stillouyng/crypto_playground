# Key Expansion _(AES-256)_

## **1. Overview**
Key Expansion is the process of deriving round keys from the original cipher key.
- **Input**: 256-bit (32-byte) master key.
- **Output**: 15 round keys (128-bit each) for 14 AES rounds + initial AddRoundKey.

---

## **2. Expand**
Function takes a **256-bit** (`32-byte`) key and **expands** it into _round keys_ for each AES round.

1. **Initialization:**
    - Vector `words` to store 32-bit words (`1 word = 4 bytes`).
    - The original `256-bit key` is split into `8 words` (each `4 bytes`).
2. **Generate new words:**
    - AES-256 requires `60 words` (15 rounds × 4 words per round).
    - For each new word (from index 8 to 59):
        - When `i % 8 == 0`:
        ```plaintext
        rot_word()
        sub_word()
        XOR with rcon (round constant)
        ```
        - When `i % 8 == 4`:
        ```plaintext
        sub_word()
        ```
      (See detailed documentation: [Converters.md](./converters.md))
        - Result: `word[i] = word[i-8] XOR temp`..
3. **Forming round keys:**
    - Group 60 words into 15 round keys of `16 bytes` each (`4 words` per key).
    - Each round key is `128 bits` (16 bytes).

---

## **3. Security Notes**
- **Non-linearity**: `sub_word` (S-Box) prevents linear cryptanalysis.
- **Diffusion**: `rot_word` + `xor_words` ensures key bits affect all round keys.
- **Round Constants**: `rcon` breaks symmetry between rounds.

---

## AES-256 Features:
1. Uses 14 rounds (plus initial round).
2. Original key - `256 bits` (`8 words`).
3. 15 round keys = 60 words.
4. Every 8th word (starting from the 8th) undergoes special processing.

---

## **Key Expansion Utilities**
Full documentation: [Converters.md](./converters.md)