#[cfg(test)]
mod tests {
    use crypto_playground::encode::GF256;

    #[test]
    fn test_add() {
        assert_eq!(GF256::add(0x57, 0x83), 0xD4);
    }

    #[test]
    fn test_mul() {
        assert_eq!(GF256::mul(0x57, 0x83), 0xC1);
        assert_eq!(GF256::mul(0x03, 0x07), 0x09);
    }

    #[test]
    fn test_reduce() {
        assert_eq!(GF256::reduce(0x1FF), 0xE4);
        assert_eq!(GF256::reduce(0x11B), 0x00);
        assert_eq!(GF256::reduce(0xFF), 0xFF);
        assert_eq!(GF256::reduce(0x100), 0x1B);
    }

    #[test]
    fn test_pow_basic() {
        assert_eq!(GF256::pow(0x03, 0), 0x01);
        assert_eq!(GF256::pow(0x03, 1), 0x03);
        assert_eq!(GF256::pow(0x03, 2), 0x05);
    }

    #[test]
    fn test_pow_with_overflow() {
        assert_eq!(GF256::pow(0x03, 4), 0x11);
    }

    #[test]
    fn test_inverse_basic() {
        assert_eq!(GF256::inverse(0x00), 0x00);
        assert_eq!(GF256::inverse(0x01), 0x01);
        assert_eq!(GF256::inverse(0x02), 0x8D);
        assert_eq!(GF256::inverse(0x03), 0xF6);
    }

    #[test]
    fn test_inverse_multiplication() {
        for a in 1..=255u16 {
            let a = a as u8;
            let inv = GF256::inverse(a);
            assert_eq!(GF256::mul(a, inv), 0x01, "Failed for a = 0x{:02X}", a);
        }
    }

}