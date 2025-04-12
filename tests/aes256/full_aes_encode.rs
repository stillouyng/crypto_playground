#[cfg(test)]
mod tests {
    use crypto_playground::encode::core::AES;
    use crypto_playground::encode::key_expansion::KeyExpansion;

    #[test]
    fn all_cycle_from_start_to_end() {

        let expected = [
            [0x8E, 0x51, 0xEA, 0x4B],
            [0xA2, 0x67, 0xFC, 0x49],
            [0xB7, 0x45, 0x49, 0x60],
            [0xCA, 0xBF, 0x90, 0x89]
        ];

        let mut state = [
            [0x00, 0x44, 0x88, 0xCC],
            [0x11, 0x55, 0x99, 0xDD],
            [0x22, 0x66, 0xAA, 0xEE],
            [0x33, 0x77, 0xBB, 0xFF]
        ];

        let key: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f
        ];

        let aes = AES::new();
        let key_expansion = KeyExpansion::new();
        let round_keys = key_expansion.expand(&key);

        aes.add_round_key(&mut state, &round_keys[0]);

        for round in 1..14 {
            aes.sub_bytes(&mut state);
            aes.shift_rows(&mut state);
            aes.mix_columns(&mut state);
            aes.add_round_key(&mut state, &round_keys[round]);
        }

        aes.sub_bytes(&mut state);
        aes.shift_rows(&mut state);
        aes.add_round_key(&mut state, &round_keys[14]);

        assert_eq!(state, expected);
    }
}