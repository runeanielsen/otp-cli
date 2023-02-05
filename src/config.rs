use std::{error::Error, fmt};

use regex::Regex;

use crate::totp::Totp;

#[derive(PartialEq, Debug, Clone)]
pub enum FormatError {
    InvalidFormat(String),
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FormatError:")
    }
}

impl Error for FormatError {}

pub fn load_totps() -> Result<Vec<Totp>, FormatError> {
    let mock_data =
        "Otpauth://totp/Acme Inc.:me@my-domain.com?secret=GZMWV5JLOMNI2XJL&issuer=AcmeCorp
Otpauth://totp/Widget Co:me@my-domain.com?secret=JXQWZ4TVRNUP5YKM&issuer=WidgetCo
Otpauth://totp/Foobar Inc.:me@my-domain.com?secret=KBYXA6USSPQ7ZLNN&issuer=FoobarInc
Otpauth://totp/Globex Corp.:me@my-domain.com?secret=LCZYB7VTTSR8AMOO&issuer=GlobexCorp
Otpauth://totp/Big Corp.:me@my-domain.com?secret=MDAZC8WUUTS9BNPP&issuer=BigCorp
Otpauth://totp/Small Firm.:me@my-domain.com?secret=NEBAD9XVVUT0COQQ&issuer=SmallFirm
Otpauth://totp/Mega Corp.:me@my-domain.com?secret=OFCAE0YWWVU1DPRR&issuer=MegaCorp
Otpauth://totp/Tech Co.:me@my-domain.com?secret=PGDBF1ZXWXU2EQSS&issuer=TechCo
Otpauth://totp/Startup Inc.:me@my-domain.com?secret=QHECK2AYXYU3FRTT&issuer=StartupInc
Otpauth://totp/Consulting Firm:me@my-domain.com?secret=RIFDL3BZYZU4GSUU&issuer=ConsultingFirm";

    mock_data
        .split('\n')
        .map(parse_google_format)
        .collect::<Result<Vec<_>, _>>()
}

fn parse_google_format(s: &str) -> Result<Totp, FormatError> {
    let re = Regex::new(r"^Otpauth://totp/(.*):.*?secret=(.*)&issuer=.*$")
        .expect("Could not parse regex.");

    if let Some(totp) = re
        .captures(s)
        .map(|captures| Totp::new(&captures[1], &captures[2], 6, 30))
    {
        Ok(totp)
    } else {
        Err(FormatError::InvalidFormat(format!(
            "Could not parse the line, invalid format: '{s}'."
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_google_authentication_format() {
        let interval = 30;
        let digits_count = 6;

        let assertions = [
            (Totp::new("Acme Inc.", "GZMWV5JLOMNI2XJL", digits_count, interval),
             "Otpauth://totp/Acme Inc.:me@my-domain.com?secret=GZMWV5JLOMNI2XJL&issuer=AcmeCorp"),
            (Totp::new("Widget Co", "JXQWZ4TVRNUP5YKM", digits_count, interval),
             "Otpauth://totp/Widget Co:me@my-domain.com?secret=JXQWZ4TVRNUP5YKM&issuer=WidgetCo"),
            (Totp::new("Foobar Inc.", "KBYXAdigits_countUSSPQ7ZLNN", digits_count, interval),
             "Otpauth://totp/Foobar Inc.:me@my-domain.com?secret=KBYXAdigits_countUSSPQ7ZLNN&issuer=FoobarInc"),
            (Totp::new("Globex Corp.", "LCZYB7VTTSR8AMOO", digits_count, interval),
             "Otpauth://totp/Globex Corp.:me@my-domain.com?secret=LCZYB7VTTSR8AMOO&issuer=GlobexCorp"),
            (Totp::new("Big Corp.", "MDAZC8WUUTS9BNPP", digits_count, interval),
             "Otpauth://totp/Big Corp.:me@my-domain.com?secret=MDAZC8WUUTS9BNPP&issuer=BigCorp"),
            (Totp::new("Small Firm.", "NEBAD9XVVUT0COQQ", digits_count, interval),
             "Otpauth://totp/Small Firm.:me@my-domain.com?secret=NEBAD9XVVUT0COQQ&issuer=SmallFirm"),
            (Totp::new("Mega Corp.", "OFCAE0YWWVU1DPRR", digits_count, interval),
             "Otpauth://totp/Mega Corp.:me@my-domain.com?secret=OFCAE0YWWVU1DPRR&issuer=MegaCorp"),
            (Totp::new("Tech Co.", "PGDBF1ZXWXU2EQSS", digits_count, interval),
             "Otpauth://totp/Tech Co.:me@my-domain.com?secret=PGDBF1ZXWXU2EQSS&issuer=TechCo"),
            (Totp::new("Startup Inc.", "QHECK2AYXYU3FRTT", digits_count, interval),
             "Otpauth://totp/Startup Inc.:me@my-domain.com?secret=QHECK2AYXYU3FRTT&issuer=StartupInc"),
            (Totp::new("Consulting Firm", "RIFDL3BZYZU4GSUU", digits_count, interval),
             "Otpauth://totp/Consulting Firm:me@my-domain.com?secret=RIFDL3BZYZU4GSUU&issuer=ConsultingFirm")];

        for (expected, input) in assertions {
            assert_eq!(Ok(expected), parse_google_format(input));
        }
    }

    #[test]
    fn invalid_google_format_error() {
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
                parse_google_format(input),
                Err(FormatError::InvalidFormat(_))
            ));
        }
    }
}
