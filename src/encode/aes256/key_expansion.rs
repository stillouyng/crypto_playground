use crate::encode::SBox;

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

    pub(crate) fn rot_word(word: [u8; 4]) -> [u8; 4] {
        [word[1], word[2], word[3], word[0]]
    }

    pub(crate) fn sub_word(&self, word: [u8; 4]) -> [u8; 4] {
        [
            self.sbox.substitute(word[0]),
            self.sbox.substitute(word[1]),
            self.sbox.substitute(word[2]),
            self.sbox.substitute(word[3]),
        ]
    }

    pub fn expand(&self, key: &[u8; 32]) -> Vec<[u8; 16]> {
        let mut words = Vec::with_capacity(60);

        for i in 0..8 {
            words.push([key[4*i], key[4*i+1], key[4*i+2], key[4*i+3]]);
        }

        for i in 8..60 {
            let mut temp = words[i-1];

            if i % 8 == 0 {
                temp = self.sub_word(Self::rot_word(temp));
                temp[0] ^= self.rcon[i/8 - 1];
            } else if i % 8 == 4 {
                temp = self.sub_word(temp);
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

fn xor_words(a: [u8; 4], b: [u8; 4]) -> [u8; 4] {
    [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
}