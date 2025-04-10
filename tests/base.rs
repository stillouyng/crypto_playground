#[cfg(test)]
mod tests {
    use crypto_playground::encode::encoding;

    #[test]
    fn test_ascii() {
        let result = encoding("A", false);
        assert_eq!(result, [65]);
    }

    #[test]
    fn test_cyrillic() {
        let result = encoding("П", false);
        assert_eq!(result, [208, 159]);
    }

    #[test]
    fn test_special_symbols() {
        let second_result = encoding("你", false);
        assert_eq!(second_result, [228, 189, 160]);
    }

    #[test]
    fn test_emojis() {
        let result = encoding("😊", false);
        assert_eq!(result, [240, 159, 152, 138]);
    }

    #[test]
    fn test_utf8_edge_cases() {
        assert_eq!(encoding("\u{0}", false), [0]);
        assert_eq!(encoding("\u{10FFFF}", false), [244, 143, 191, 191]);
    }

    #[test]
    fn test_ascii_vs_unicode() {
        assert_eq!(
            encoding("AП€🐱", false),
            [65, 208, 159, 226, 130, 172, 240, 159, 144, 177]
        );
    }

    #[test]
    fn test_stress() {
        let smile = "😊";
        let big_string = smile.repeat(1000);

        assert_eq!(
            encoding(&*big_string, false),
            [240, 159, 152, 138].repeat(1000)
        );
    }

}
