#[cfg(test)]
mod tests {
    use crypto_playground::encode::key_expansion::KeyExpansion;

    #[test]
    fn test_key_expansion_length() {
        let key = [0u8; 32];
        let expanded_keys = KeyExpansion::new().expand(&key);

        assert_eq!(expanded_keys.len(), 15);
        assert!(expanded_keys.iter().all(|k| k.len() == 16));
    }

    #[test]
    fn test_first_and_last_round_keys() {
        let key = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
            0x1C, 0x1D, 0x1E, 0x1F,
        ];
        let expanded_keys = KeyExpansion::new().expand(&key);

        assert_eq!(
            expanded_keys[0],
            <&[u8] as TryInto<[u8; 16]>>::try_into(&key[..16]).unwrap()
        );

        assert_ne!(expanded_keys[0], expanded_keys[14]);
    }

    #[test]
    fn test_nist_test_vector() {
        let key: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
            0x1C, 0x1D, 0x1E, 0x1F,
        ];

        let expanded_keys = KeyExpansion::new().expand(&key);
        /*
            In AES-256, the key schedule starts with 8 words (32 bytes) from the input key.
            The actual "round keys" used in encryption begin from word 8 (`w[8]`) onward.
            Therefore, the first round key is composed of `w[8] w[9] w[10] w[11]`,
            which in our implementation is `expanded_keys[2]`.
        */
        assert_eq!(
            expanded_keys[2],
            [
                0xA5, 0x73, 0xC2, 0x9F, 0xA1, 0x76, 0xC4, 0x98, 0xA9, 0x7F, 0xCE, 0x93, 0xA5, 0x72,
                0xC0, 0x9C
            ]
        );
    }

    #[test]
    fn test_all_zero_key() {
        let key = [0u8; 32];
        let expanded_keys = KeyExpansion::new().expand(&key);

        let resp_false = expanded_keys.iter().all(|k| k != &[0u8; 16]);
        assert_eq!(resp_false, false);
        /*
           The same logic as it was in previous test.
           Not-null values we'll got in first round key
           which is `expanded_keys[2]` etc..
        */
        let resp_true = expanded_keys[2..].iter().all(|k| k != &[0u8; 16]);
        assert!(resp_true);
    }

    #[test]
    fn test_all_ff_key() {
        let key = [0xFFu8; 32];
        let expanded_keys = KeyExpansion::new().expand(&key);

        assert_ne!(expanded_keys[0], expanded_keys[14]);
    }
}
