
#[cfg(test)]
mod tests {
    use crypto_playground::encode::{xor_encrypt, generate_key};

    #[test]
    #[ignore = "Works with files. Will not be success when tested with CI"]
    fn test_generate_key_length() {
        let key = generate_key(32);
        assert_eq!(key.len(), 32); // Проверяем длину
    }

    #[test]
    #[ignore = "Works with files. Will not be success when tested with CI"]
    fn test_generate_key_randomness() {
        let key1 = generate_key(16);
        let key2 = generate_key(16);
        assert_ne!(key1, key2);
    }

    #[test]
    #[ignore = "Works with files. Will not be success when tested with CI"]
    fn test_xor_empty_input() {
        let empty: &[u8] = &[];
        let result = xor_encrypt(empty);
        assert!(result.is_empty());
    }
}
