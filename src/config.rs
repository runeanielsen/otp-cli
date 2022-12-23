use crate::totp::Totp;

pub fn longest_name_char_count(configs: &[Totp]) -> Option<usize> {
    configs
        .iter()
        .max_by(|x, y| x.name.len().cmp(&y.name.len()))
        .map(|config| config.name.len())
}

pub fn max_digits(configs: &[Totp]) -> Option<u32> {
    configs
        .iter()
        .max_by(|x, y| x.digits.cmp(&y.digits))
        .map(|config| config.digits)
}

pub fn load() -> Vec<Totp> {
    vec![
        Totp::new("Acme Inc.", "8n4mzt7w", 6, 30),
        Totp::new("Gizmo Corporation", "xkc2j8fh", 6, 30),
        Totp::new("Foo Industries", "9s6bk3jq", 6, 30),
        Totp::new("Bar Enterprises", "7h1lm5rp", 6, 30),
        Totp::new("Baz Inc.", "2v9d4k8c", 6, 30),
        Totp::new("Qux Limited", "5j6w7m2v", 6, 30),
        Totp::new("Quux Corp.", "3p8s1q9z", 6, 30),
        Totp::new("Corge Enterprises", "4y7e2u5k", 6, 30),
        Totp::new("Grault Inc.", "6f9h2l5m", 6, 30),
        Totp::new("Garply Co.", "1d4t7h2v", 6, 30),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_longest_config_name_by_count() {
        let configs = [
            Totp::new("Acme Inc.", "8n4mzt7w", 6, 30),
            Totp::new("Gizmo Corporation", "xkc2j8fh", 6, 30),
            Totp::new("Foo Industries", "9s6bk3jq", 6, 30),
            Totp::new("Bar Enterprises", "7h1lm5rp", 6, 30),
            Totp::new("Baz Inc.", "2v9d4k8c", 6, 30),
            Totp::new("Qux Limited", "5j6w7m2v", 6, 30),
            Totp::new("Quux Corp.", "3p8s1q9z", 6, 30),
            Totp::new("Corge Enterprises", "4y7e2u5k", 6, 30),
            Totp::new("Grault Inc.", "6f9h2l5m", 6, 30),
            Totp::new("Garply Co.", "1d4t7h2v", 6, 30),
        ];

        assert_eq!(17, longest_name_char_count(&configs).unwrap());
    }

    #[test]
    fn can_find_most_otp_digits_in_configs() {
        let configs = [
            Totp::new("Acme Inc.", "8n4mzt7w", 6, 30),
            Totp::new("Gizmo Corporation", "xkc2j8fh", 6, 30),
            Totp::new("Foo Industries", "9s6bk3jq", 6, 30),
            Totp::new("Bar Enterprises", "7h1lm5rp", 6, 30),
            Totp::new("Baz Inc.", "2v9d4k8c", 6, 30),
            Totp::new("Qux Limited", "5j6w7m2v", 6, 30),
            Totp::new("Quux Corp.", "3p8s1q9z", 6, 30),
            Totp::new("Corge Enterprises", "4y7e2u5k", 6, 30),
            Totp::new("Grault Inc.", "6f9h2l5m", 6, 30),
            Totp::new("Garply Co.", "1d4t7h2v", 6, 30),
        ];

        assert_eq!(6, max_digits(&configs).unwrap());
    }
}
