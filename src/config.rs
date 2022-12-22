#[derive(Clone)]
pub struct Config {
    pub name: String,
    pub secret: String,
    pub digits: u32,
    pub interval: u64,
}

impl Config {
    pub fn new(name: &str, secret: &str, digits: u32, interval: u64) -> Self {
        Self {
            name: name.to_string(),
            secret: secret.to_string(),
            digits,
            interval,
        }
    }
}

pub fn longest_name_char_count(configs: &[Config]) -> Option<usize> {
    configs
        .iter()
        .max_by(|x, y| x.name.len().cmp(&y.name.len()))
        .map(|config| config.name.len())
}

pub fn max_digits(configs: &[Config]) -> Option<u32> {
    configs
        .iter()
        .max_by(|x, y| x.digits.cmp(&y.digits))
        .map(|config| config.digits)
}

pub fn load() -> Vec<Config> {
    vec![
        Config::new("Codeberg 1", "hello1", 6, 30),
        Config::new("Codeberg 2", "hello2", 6, 30),
        Config::new("Codeberg 3", "hello3", 6, 30),
        Config::new("Codeberg 4", "hello4", 6, 30),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_longest_config_name_by_count() {
        let configs = [
            Config {
                name: "Mastodon".to_string(),
                secret: "hello".to_string(),
                interval: 30,
                digits: 6,
            },
            Config {
                name: "Codeberg".to_string(),
                secret: "hello world!".to_string(),
                interval: 30,
                digits: 6,
            },
            Config {
                name: "Codeberg 1".to_string(),
                secret: "hello world!1".to_string(),
                interval: 30,
                digits: 6,
            },
            Config {
                name: "This is a very long name".to_string(),
                secret: "hello world!1".to_string(),
                interval: 30,
                digits: 6,
            },
            Config {
                name: "Codeberg 2".to_string(),
                secret: "hello world!2".to_string(),
                interval: 60,
                digits: 6,
            },
            Config {
                name: "Codeberg 3".to_string(),
                secret: "hello world!3".to_string(),
                interval: 30,
                digits: 8,
            },
        ];

        assert_eq!(24, longest_name_char_count(&configs).unwrap());
    }

    #[test]
    fn can_find_most_otp_digits_in_configs() {
        let configs = [
            Config {
                name: "Mastodon".to_string(),
                secret: "hello".to_string(),
                interval: 30,
                digits: 6,
            },
            Config {
                name: "Codeberg".to_string(),
                secret: "hello world!".to_string(),
                interval: 30,
                digits: 6,
            },
            Config {
                name: "Codeberg 1".to_string(),
                secret: "hello world!1".to_string(),
                interval: 30,
                digits: 6,
            },
            Config {
                name: "This is a very long name".to_string(),
                secret: "hello world!1".to_string(),
                interval: 30,
                digits: 6,
            },
            Config {
                name: "Codeberg 2".to_string(),
                secret: "hello world!2".to_string(),
                interval: 60,
                digits: 6,
            },
            Config {
                name: "Codeberg 3".to_string(),
                secret: "hello world!3".to_string(),
                interval: 30,
                digits: 8,
            },
        ];

        assert_eq!(8, max_digits(&configs).unwrap());
    }
}
