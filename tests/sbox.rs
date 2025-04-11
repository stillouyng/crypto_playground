#[cfg(test)]
mod tests {
    use crypto_playground::encode::GF256;
    use crypto_playground::encode::SBox;

    #[test]
    fn test_affine_transform_zero() {
        assert_eq!(SBox::affine_transform(0x00), 0x63);
    }

    #[test]
    fn test_affine_transform_known_values() {
        assert_eq!(SBox::affine_transform(GF256::inverse(0x00)), 0x63);
        assert_eq!(SBox::affine_transform(GF256::inverse(0xFF)), 0x16);
        assert_eq!(SBox::affine_transform(GF256::inverse(0x01)), 0x7C);
    }


    #[test]
    fn test_sbox_values() {
        let sbox = SBox::new();
        assert_eq!(sbox.substitute(0x00), 0x63);
        assert_eq!(sbox.substitute(0x53), 0xED);
        assert_eq!(sbox.substitute(0xFF), 0x16);
    }

}