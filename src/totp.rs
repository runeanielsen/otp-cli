use std::time::SystemTime;

use base32::Alphabet;
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

    pub fn code_padded(&self, time: SystemTime) -> String {
        format!(
            "{:0digits_width$}",
            self.code(time),
            digits_width = self.digits as usize
        )
    }

    fn code(&self, time: SystemTime) -> u32 {
        let counter = time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / self.interval;

        let decoded_secret = base32::decode(
            Alphabet::RFC4648 { padding: false },
            &self.secret.to_ascii_uppercase(),
        )
        .unwrap();

        let digest = Hmac::<Sha1>::new_from_slice(&decoded_secret)
            .unwrap()
            .chain_update(counter.to_be_bytes())
            .finalize()
            .into_bytes();

        let offset = (digest[19] & 0xf) as usize;
        let code = u32::from_be_bytes([
            digest[offset] & 0x7f,
            digest[offset + 1],
            digest[offset + 2],
            digest[offset + 3],
        ] as [u8; 4]);

        code % 10_u32.pow(self.digits)
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
        let march_14_2020 = SystemTime::UNIX_EPOCH + Duration::new(1_584_140_400, 0);

        let test_data = [
            (
                954_926,
                Totp::new("Acme Inc.", "NBSWY3DPEBXWIZLOMUVZWK4Q=", 6, 30),
            ),
            (711_370, Totp::new("Gizmo Corporation", "MFRGGZDF", 6, 30)),
            (
                672_595,
                Totp::new("Foo Industries", "MZXW6YTBOI======", 6, 30),
            ),
            (
                333_890,
                Totp::new("Bar Enterprises", "JBSWY3DPFQQFO33SNRSCC===", 6, 30),
            ),
        ];

        for (expected, totp) in test_data {
            assert_eq!(expected, totp.code(march_14_2020));
        }
    }
}
