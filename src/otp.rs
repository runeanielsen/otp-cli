use std::time::SystemTime;

use hmac::{Hmac, Mac};
use sha1::Sha1;

fn step_counter(time: &SystemTime, step: u64) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / step
}

/// HMAC-based one-time password for a given secret and counter.
fn hotp(secret: &str, counter: u64) -> u32 {
    let digest = Hmac::<Sha1>::new_from_slice(&secret.to_owned().into_bytes())
        .unwrap()
        .chain_update(counter.to_be_bytes())
        .finalize()
        .into_bytes();

    let offset = (digest[19] & 0xf) as usize;
    let code = u32::from(digest[offset] & 0x7f) << 24
        | u32::from(digest[(offset + 1)]) << 16
        | u32::from(digest[(offset + 2)]) << 8
        | u32::from(digest[(offset + 3)]);

    code % 1_000_000
}

pub fn totp(secret: &str, interval: u64) -> u32 {
    hotp(secret, step_counter(&SystemTime::now(), interval))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_totp() {
        // Want make sure that something is returned.
        // We cannot, assert on specific values, since the current time
        // changes all the time.
        assert!(totp("hello", 30) > 0);
    }

    #[test]
    fn get_hotp() {
        let assertions = [
            (124_111, "Hello world!"),
            (654_079, "it works!"),
            (722_283, "yes it does!"),
        ];

        for (expected, secret) in assertions {
            let result = hotp(secret, 0);
            assert_eq!(expected, result);
        }
    }
}
