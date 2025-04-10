pub struct GF256;


impl GF256 {

    pub fn add(a: u8, b: u8) -> u8 {
        a ^ b
    }

    pub fn xtime(a: u8) -> u8 {
        let hi_bit = (a & 0x80) != 0;
        let mut res = a << 1;
        if hi_bit {
            res ^= 0x1B;
        }
        res
    }

    pub fn mul(a: u8, b: u8) -> u8 {
        let mut result = 0;
        let mut current = a;

        for i in 0..8 {
            if (b >> i) & 1 == 1 {
                result ^= current;
            }
            current = Self::xtime(current);
        }

        result
    }

    pub fn reduce(poly: u16) -> u8 {
        let mut poly = poly;
        while poly > 0xFF {
            let degree = 15 - poly.leading_zeros() as u8;
            let shift = degree - 8;
            poly ^= 0x11B << shift;
        }
        poly as u8
    }
}