use crate::encode::SBox;
use crate::encode::GF256;


pub struct AES {
    sbox: SBox
}

impl AES {
    pub fn new() -> Self {
        AES {
            sbox: SBox::new()
        }
    }

    pub fn shift_rows(&self, state: &mut [[u8; 4]; 4]) {
        for i in 1..4 {
            state[i].rotate_left(i);
        }
    }

    pub fn mix_columns(&self, state: &mut [[u8; 4]; 4]) {
        for col in 0..4 {
            let a0 = state[0][col];
            let a1 = state[1][col];
            let a2 = state[2][col];
            let a3 = state[3][col];

            state[0][col] = GF256::mul(0x02, a0) ^ GF256::mul(0x03, a1) ^ a2 ^ a3;
            state[1][col] = a0 ^ GF256::mul(0x02, a1) ^ GF256::mul(0x03, a2) ^ a3;
            state[2][col] = a0 ^ a1 ^ GF256::mul(0x02, a2) ^ GF256::mul(0x03, a3);
            state[3][col] = GF256::mul(0x03, a0) ^ a1 ^ a2 ^ GF256::mul(0x02, a3);
        }
    }
}