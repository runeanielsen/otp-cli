use std::time::SystemTime;

use hmac::{Hmac, Mac};
use sha1::Sha1;

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

    pub fn code(&self, time: &SystemTime) -> u32 {
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

    // Duration used so far for TOTP.
    pub fn duration_used(&self, time: &SystemTime) -> u64 {
        time.duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            % self.interval
    }
}
