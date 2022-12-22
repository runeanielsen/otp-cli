use std::time::SystemTime;

use hmac::{Hmac, Mac};
use sha1::Sha1;

/// Counts the steps since unix epoch.
fn step_counter(time: &SystemTime, step: u64) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / step
}

/// HMAC-based one-time password for a given secret and counter.
fn hotp(secret: &str, counter: u64, digits: u32) -> u32 {
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

    code % (10_u32).pow(digits)
}

pub fn totp(secret: &str, interval: u64, digits: u32) -> u32 {
    hotp(secret, step_counter(&SystemTime::now(), interval), digits)
}

// Duration used so far for TOTP.
pub fn duration_used(time: &SystemTime, step: u64) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % step
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_totp() {
        // Want make sure that something is returned.
        // We cannot, assert on specific values, since the current time
        // changes all the time.
        assert!(totp("hello", 30, 6) > 0);
        assert!(totp("sdfsfs", 15, 6) > 0);
        assert!(totp("wowi", 10, 6) > 0);
        assert!(totp("mysuperdupersecret#!@#", 2000, 6) > 0);
    }

    #[test]
    fn get_hotp() {
        assert_eq!(124_111, hotp("Hello world!", 0, 6));
        assert_eq!(654_079, hotp("it works!", 0, 6));
        assert_eq!(722_283, hotp("yes it does!", 0, 6));

        assert_eq!(42_124_111, hotp("Hello world!", 0, 8));
        assert_eq!(47_654_079, hotp("it works!", 0, 8));
        assert_eq!(19_722_283, hotp("yes it does!", 0, 8));
    }
}
