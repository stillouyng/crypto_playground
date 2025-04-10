use crate::encode::gf256::GF256;

pub struct SBox {
    pub table: [u8; 256],
    // pub inverse: [u8; 256],
}

impl SBox {
    pub fn new() -> Self {
        let mut table = [0; 256];
        for a in 0..=255 {
            let inv = GF256::inverse(a);
            table[a as usize] = Self::affine_transform(inv);
        }
        SBox { table }
    }

    pub fn affine_transform(byte: u8) -> u8 {
        let mut result = 0u8;
        for i in 0..8 {
            let bit = ((byte >> i) & 1) ^
                ((byte >> ((i + 4) % 8)) & 1) ^
                ((byte >> ((i + 5) % 8)) & 1) ^
                ((byte >> ((i + 6) % 8)) & 1) ^
                ((byte >> ((i + 7) % 8)) & 1) ^
                ((0x63 >> i) & 1);
            result |= bit << i;
        }
        result
    }

    pub fn substitute(&self, byte: u8) -> u8 {
        self.table[byte as usize]
    }
}
