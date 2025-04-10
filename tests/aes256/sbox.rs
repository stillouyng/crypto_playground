#[cfg(test)]
mod tests {
    use crypto_playground::encode::{sbox::SBox, gf256::GF256};

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
}