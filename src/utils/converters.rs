use crate::encode::SBox;


pub const fn rot_word(word: [u8; 4]) -> [u8; 4] {
    [word[1], word[2], word[3], word[0]]
}

pub fn sub_word(sbox: &SBox, word: [u8; 4]) -> [u8; 4] {
    [
        sbox.substitute(word[0]),
        sbox.substitute(word[1]),
        sbox.substitute(word[2]),
        sbox.substitute(word[3]),
    ]
}

pub fn xor_words(a: [u8; 4], b: [u8; 4]) -> [u8; 4] {
    [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
}