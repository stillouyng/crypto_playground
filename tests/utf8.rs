#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};
    use crypto_playground::encode::encode;

    #[test]
    fn test_encode_is_just_bytes() {
        let phrase = "Hello 🦀";
        assert_eq!(encode(phrase), phrase.as_bytes());
    }

    #[test]
    fn random_unicode_stress_test() {
        let mut rng = thread_rng();

        for _ in 0..10000 {
            let code = rng.gen_range(0..=0x10FFFF);
            if let Some(c) = char::from_u32(code) {
                let s = c.to_string();
                assert_eq!(encode(&s), s.as_bytes());
            }
        }
    }
}