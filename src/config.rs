use regex::Regex;

use crate::totp::Totp;

pub fn load() -> Vec<Option<Totp>> {
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

    mock_data.split('\n').map(parse_google_format).collect()
}

fn parse_google_format(s: &str) -> Option<Totp> {
    let re = Regex::new(r"^Otpauth://totp/(.*):.*?secret=(.*)&issuer=.*$")
        .expect("Could not parse regex.");

    re.captures(s)
        .map(|captures| Totp::new(&captures[1], &captures[2], 6, 30))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let xs = [
            (Totp::new("Acme Inc.", "GZMWV5JLOMNI2XJL", 6, 30),
             "Otpauth://totp/Acme Inc.:me@my-domain.com?secret=GZMWV5JLOMNI2XJL&issuer=AcmeCorp"),
            (Totp::new("Widget Co", "JXQWZ4TVRNUP5YKM", 6, 30),
             "Otpauth://totp/Widget Co:me@my-domain.com?secret=JXQWZ4TVRNUP5YKM&issuer=WidgetCo"),
            (Totp::new("Foobar Inc.", "KBYXA6USSPQ7ZLNN", 6, 30),
             "Otpauth://totp/Foobar Inc.:me@my-domain.com?secret=KBYXA6USSPQ7ZLNN&issuer=FoobarInc"),
            (Totp::new("Globex Corp.", "LCZYB7VTTSR8AMOO", 6, 30),
             "Otpauth://totp/Globex Corp.:me@my-domain.com?secret=LCZYB7VTTSR8AMOO&issuer=GlobexCorp"),
            (Totp::new("Big Corp.", "MDAZC8WUUTS9BNPP", 6, 30),
             "Otpauth://totp/Big Corp.:me@my-domain.com?secret=MDAZC8WUUTS9BNPP&issuer=BigCorp"),
            (Totp::new("Small Firm.", "NEBAD9XVVUT0COQQ", 6, 30),
             "Otpauth://totp/Small Firm.:me@my-domain.com?secret=NEBAD9XVVUT0COQQ&issuer=SmallFirm"),
            (Totp::new("Mega Corp.", "OFCAE0YWWVU1DPRR", 6, 30),
             "Otpauth://totp/Mega Corp.:me@my-domain.com?secret=OFCAE0YWWVU1DPRR&issuer=MegaCorp"),
            (Totp::new("Tech Co.", "PGDBF1ZXWXU2EQSS", 6, 30),
             "Otpauth://totp/Tech Co.:me@my-domain.com?secret=PGDBF1ZXWXU2EQSS&issuer=TechCo"),
            (Totp::new("Startup Inc.", "QHECK2AYXYU3FRTT", 6, 30),
             "Otpauth://totp/Startup Inc.:me@my-domain.com?secret=QHECK2AYXYU3FRTT&issuer=StartupInc"),
            (Totp::new("Consulting Firm", "RIFDL3BZYZU4GSUU", 6, 30),
             "Otpauth://totp/Consulting Firm:me@my-domain.com?secret=RIFDL3BZYZU4GSUU&issuer=ConsultingFirm")];

        for (expected, input) in xs {
            assert_eq!(Some(expected), parse_google_format(input));
        }
    }
}
