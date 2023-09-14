use std::{convert::Into, error::Error, fmt, fs, io::ErrorKind, path::PathBuf};

use regex::Regex;

use crate::totp::Totp;

#[derive(PartialEq, Debug, Clone)]
pub enum TotpSecretFileError {
    NotFound(String),
    InvalidFormat(String),
}

impl fmt::Display for TotpSecretFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TotpSecretFileError::NotFound(message)
                | TotpSecretFileError::InvalidFormat(message) => message,
            }
        )
    }
}

impl Error for TotpSecretFileError {}

pub fn load_totps(
    config_dir_path: PathBuf,
    config_file_name: &str,
    digits: u32,
    interval: u64,
) -> Result<Vec<Totp>, Box<dyn Error>> {
    let totps_file_path: PathBuf = [config_dir_path, PathBuf::from(config_file_name)]
        .iter()
        .collect();

    let secret_file_content = match fs::read_to_string(&totps_file_path) {
        Ok(file_content) => Ok(file_content),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => Err(TotpSecretFileError::NotFound(format!(
                "Could not find TOTP secret file '{}'.",
                totps_file_path
                    .to_str()
                    .expect("Could not convert TOTP secret file-path to valid UTF-8.")
            ))),
            unhandled_err => {
                panic!(
                    "Problem opening the TOTP-secrets file '{}': '{unhandled_err}'.",
                    totps_file_path
                        .to_str()
                        .expect("Could not convert TOTP secret file-path to valid UTF-8.")
                );
            }
        },
    }?;

    secret_file_content
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(str::trim)
        .map(|x| parse_uri_string_format(x, digits, interval).map_err(Into::into))
        .collect()
}

fn parse_uri_string_format(
    s: &str,
    digits: u32,
    interval: u64,
) -> Result<Totp, TotpSecretFileError> {
    let re = Regex::new(r"(?i)^otpauth://totp/(.*):.*?secret=(.*)&issuer=.*$")
        .expect("Could not parse regex.");

    if let Some(totp) = re
        .captures(s)
        .map(|captures| Totp::new(&captures[1], &captures[2], digits, interval))
    {
        Ok(totp)
    } else {
        Err(TotpSecretFileError::InvalidFormat(format!(
            "Could not parse the line, invalid format: '{s}'."
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_uri_string_format() {
        let interval = 30;
        let digits = 6;

        let assertions = [
            (Totp::new("Acme Inc.", "GZMWV5JLOMNI2XJL", digits, interval),
             "Otpauth://totp/Acme Inc.:me@my-domain.com?secret=GZMWV5JLOMNI2XJL&issuer=AcmeCorp"),
            (Totp::new("Widget Co", "JXQWZ4TVRNUP5YKM", digits, interval),
             "Otpauth://totp/Widget Co:me@my-domain.com?secret=JXQWZ4TVRNUP5YKM&issuer=WidgetCo"),
            (Totp::new("Foobar Inc.", "KBYXAdigits_countUSSPQ7ZLNN", digits, interval),
             "Otpauth://totp/Foobar Inc.:me@my-domain.com?secret=KBYXAdigits_countUSSPQ7ZLNN&issuer=FoobarInc"),
            (Totp::new("Globex Corp.", "LCZYB7VTTSR8AMOO", digits, interval),
             "Otpauth://totp/Globex Corp.:me@my-domain.com?secret=LCZYB7VTTSR8AMOO&issuer=GlobexCorp"),
            (Totp::new("Big Corp.", "MDAZC8WUUTS9BNPP", digits, interval),
             "Otpauth://totp/Big Corp.:me@my-domain.com?secret=MDAZC8WUUTS9BNPP&issuer=BigCorp"),
            (Totp::new("Small Firm.", "NEBAD9XVVUT0COQQ", digits, interval),
             "Otpauth://totp/Small Firm.:me@my-domain.com?secret=NEBAD9XVVUT0COQQ&issuer=SmallFirm"),
            (Totp::new("Mega Corp.", "OFCAE0YWWVU1DPRR", digits, interval),
             "Otpauth://totp/Mega Corp.:me@my-domain.com?secret=OFCAE0YWWVU1DPRR&issuer=MegaCorp"),
            (Totp::new("Tech Co.", "PGDBF1ZXWXU2EQSS", digits, interval),
             "Otpauth://totp/Tech Co.:me@my-domain.com?secret=PGDBF1ZXWXU2EQSS&issuer=TechCo"),
            (Totp::new("Startup Inc.", "QHECK2AYXYU3FRTT", digits, interval),
             "Otpauth://totp/Startup Inc.:me@my-domain.com?secret=QHECK2AYXYU3FRTT&issuer=StartupInc"),
            (Totp::new("Consulting Firm", "RIFDL3BZYZU4GSUU", digits, interval),
             "Otpauth://totp/Consulting Firm:me@my-domain.com?secret=RIFDL3BZYZU4GSUU&issuer=ConsultingFirm")];

        for (expected, input) in assertions {
            assert_eq!(
                Ok(expected),
                parse_uri_string_format(input, digits, interval)
            );
        }
    }

    #[test]
    fn invalid_uri_string_format_results_in_invalid_format_error() {
        let interval = 30;
        let digits = 6;

        let assertions = [
            "",
            "qwerty",
            "Otpauth://totp/Mega Corp.",
            "tpauth://totp/Acme Inc.:me@my-domain.com?secret=GZMWV5JLOMNI2XJL&issuer=AcmeCorp",
            "Otpaut://totp/Widget Co:me@my-domain.com?secret=JXQWZ4TVRNUP5YKM&issuer=WidgetCo",
            "Otputh://totp/Foobar Inc.:me@my-domain.com?secret=KBYXAdigits_countUSSPQ7ZLNN&issuer=FoobarInc",
            "Otpauth://Globex Corp.:me@my-domain.com?secret=LCZYB7VTTSR8AMOO&issuer=GlobexCorp",
            "Otpauth://totp/Big Corp.:me@my-domain.com&issuer=BigCorp",
            "Otpauth://totp/secret=NEBAD9XVVUT0COQQ&issuer=SmallFirm",
            "Otpauth://totp/Mega Corp.:me@my-domain.com?secret=OFCAE0YWWVU1DPRR"];

        for input in assertions {
            assert!(matches!(
                parse_uri_string_format(input, digits, interval),
                Err(TotpSecretFileError::InvalidFormat(_))
            ));
        }
    }
}
