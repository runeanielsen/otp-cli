use std::time::SystemTime;

use hmac::{Hmac, Mac};
use sha1::Sha1;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Totp {
    pub name: String,
    secret: String,
    pub digits: u32,
    pub interval: u64,
}

impl Totp {
    pub fn new(name: &str, secret: &str, digits: u32, interval: u64) -> Self {
        Self {
            name: name.to_string(),
            secret: secret.to_string(),
            digits,
            interval,
        }
    }

    pub fn code(&self, time: SystemTime) -> u32 {
        let counter = time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / self.interval;

        let digest = Hmac::<Sha1>::new_from_slice(&self.secret.clone().into_bytes())
            .unwrap()
            .chain_update(counter.to_be_bytes())
            .finalize()
            .into_bytes();

        let offset = (digest[19] & 0xf) as usize;
        let code = u32::from(digest[offset] & 0x7f) << 24
            | u32::from(digest[(offset + 1)]) << 16
            | u32::from(digest[(offset + 2)]) << 8
            | u32::from(digest[(offset + 3)]);

        code % (10_u32).pow(self.digits)
    }
}

pub fn duration_used(interval: u64, time: SystemTime) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % interval
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn totp_code_calculation_is_correct() {
        let march_14_2020 = SystemTime::UNIX_EPOCH + Duration::new(1_584_188_800, 0);

        let test_data = [
            (640_572, Totp::new("Acme Inc.", "8n4mzt7w", 6, 30)),
            (87439, Totp::new("Gizmo Corporation", "xkc2j8fh", 6, 30)),
            (771_990, Totp::new("Foo Industries", "9s6bk3jq", 6, 30)),
            (438_943, Totp::new("Bar Enterprises", "7h1lm5rp", 6, 30)),
            (52859, Totp::new("Baz Inc.", "2v9d4k8c", 6, 30)),
            (669_413, Totp::new("Qux Limited", "5j6w7m2v", 6, 30)),
            (203_698, Totp::new("Quux Corp.", "3p8s1q9z", 6, 30)),
            (640_828, Totp::new("Corge Enterprises", "4y7e2u5k", 6, 30)),
            (619_356, Totp::new("Grault Inc.", "6f9h2l5m", 6, 30)),
            (73510, Totp::new("Garply Co.", "1d4t7h2v", 6, 30)),
        ];

        for (expected, totp) in test_data {
            assert_eq!(expected, totp.code(march_14_2020));
        }
    }
}
