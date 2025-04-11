#[cfg(test)]
mod tests {
    use crypto_playground::encode::SBox;
    use crypto_playground::utils::{rot_word, sub_word, xor_words};

    #[test]
    fn test_rot_word() {
        assert_eq!(rot_word([0x01, 0x02, 0x03, 0x04]), [0x02, 0x03, 0x04, 0x01]);
    }

    #[test]
    fn test_sub_word() {
        let sbox = SBox::new();
        assert_eq!(sub_word(&sbox, [0x00, 0x01, 0x02, 0x03]), [
            sbox.substitute(0x00),
            sbox.substitute(0x01),
            sbox.substitute(0x02),
            sbox.substitute(0x03)
        ]);
    }

    #[test]
    fn test_xor_words() {
        assert_eq!(xor_words([0xFF, 0x00, 0xAA, 0x55], [0xAA, 0x55, 0xFF, 0x00]),
                   [0x55, 0x55, 0x55, 0x55]);
    }
}