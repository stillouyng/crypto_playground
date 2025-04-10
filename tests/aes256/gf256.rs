#[cfg(test)]
mod tests {
    use crypto_playground::encode::gf256::GF256;

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
}