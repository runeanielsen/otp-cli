use hmac::{Hmac, Mac};
use sha1::Sha1;

type HmacSha1 = Hmac<Sha1>;

/// HMAC-based one-time password for a given secret and counter.
fn hotp(secret: &str, counter: u64) -> u32 {
    let digest = HmacSha1::new_from_slice(&secret.to_owned().into_bytes())
        .unwrap()
        .chain_update(counter.to_be_bytes())
        .finalize()
        .into_bytes();

    let offset = (digest[19] & 0xf) as usize;
    let code: u32 = u32::from(digest[offset] & 0x7f) << 24
        | u32::from(digest[(offset + 1)]) << 16
        | u32::from(digest[(offset + 2)]) << 8
        | u32::from(digest[(offset + 3)]);

    code % 1_000_000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_hotp_no_counter() {
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
