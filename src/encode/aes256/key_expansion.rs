use crate::encode::SBox;
use crate::utils::{sub_word, rot_word, xor_words};

pub struct KeyExpansion {
    sbox: SBox,
    rcon: [u8; 10]
}


impl KeyExpansion {
    pub fn new() -> Self {
        KeyExpansion {
            sbox: SBox::new(),
            rcon: [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36]
        }
    }

    pub fn expand(&self, key: &[u8; 32]) -> Vec<[u8; 16]> {
        let mut words = Vec::with_capacity(60);

        for i in 0..8 {
            words.push([key[4*i], key[4*i+1], key[4*i+2], key[4*i+3]]);
        }

        for i in 8..60 {
            let mut temp = words[i-1];

            if i % 8 == 0 {
                temp = sub_word(&self.sbox, rot_word(temp));
                temp[0] ^= self.rcon[i/8 - 1];
            } else if i % 8 == 4 {
                temp = sub_word(&self.sbox, temp);
            }

            let new_word = xor_words(words[i-8], temp);
            words.push(new_word);
        }

        (0..15).map(|i| {
            let mut round_key = [0; 16];
            for j in 0..4 {
                round_key[4*j..4*j+4].copy_from_slice(&words[4*i + j]);
            }
            round_key
        }).collect()
    }
}