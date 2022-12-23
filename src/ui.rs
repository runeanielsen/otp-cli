use std::{
    error::Error,
    io::Write,
    time::{Duration, SystemTime},
};

use crossterm::{
    cursor,
    event::{self, poll, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
};

use crate::totp::Totp;

fn longest_name_char_count(configs: &[Totp]) -> Option<usize> {
    configs
        .iter()
        .max_by(|x, y| x.name.len().cmp(&y.name.len()))
        .map(|config| config.name.len())
}

fn format_totps(configs: &[Totp], time: &SystemTime) -> String {
    configs
        .iter()
        .map(|x| {
            format!(
                "{:<max_length$} | {:0digits_width$} | {:02}/{}\n",
                x.name,
                x.code(time),
                x.duration_used(time),
                x.interval,
                digits_width = x.digits as usize,
                max_length = longest_name_char_count(configs).unwrap()
            )
        })
        .collect::<String>()
}

fn read_char() -> Option<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Some(c);
        }
    }
}

pub fn start<W>(w: &mut W, configs: &[Totp]) -> Result<(), Box<dyn Error>>
where
    W: Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    loop {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        for line in format_totps(configs, &SystemTime::now()).split('\n') {
            queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        w.flush()?;

        if poll(Duration::from_millis(1_000))? {
            if let 'q' = read_char().expect("Could not read input.") {
                break;
            };
        }
    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_totp() {
        let totps = [
            Totp::new("Acme Inc.", "8n4mzt7w", 6, 30),
            Totp::new("Gizmo Corporation", "xkc2j8fh", 6, 30),
            Totp::new("Foo Industries", "9s6bk3jq", 6, 30),
            Totp::new("Bar Enterprises", "7h1lm5rp", 6, 30),
            Totp::new("Baz Inc.", "2v9d4k8c", 6, 30),
            Totp::new("Qux Limited", "5j6w7m2v", 6, 30),
            Totp::new("Quux Corp.", "3p8s1q9z", 6, 30),
            Totp::new("Corge Enterprises", "4y7e2u5k", 6, 30),
            Totp::new("Grault Inc.", "6f9h2l5m", 6, 30),
            Totp::new("Garply Co.", "1d4t7h2v", 6, 30),
        ];

        assert_eq!(17, longest_name_char_count(&totps).unwrap());
    }

    #[test]
    fn totp_is_formatted_correctly() {
        let march_14_2020 = SystemTime::UNIX_EPOCH + Duration::new(1_584_188_800, 0);
        let totps = [
            Totp::new("Acme Inc.", "8n4mzt7w", 6, 30),
            Totp::new("Gizmo Corporation", "xkc2j8fh", 6, 30),
            Totp::new("Foo Industries", "9s6bk3jq", 6, 30),
        ];

        let expected = "Acme Inc.         | 640572 | 10/30\nGizmo Corporation | 087439 | 10/30\nFoo Industries    | 771990 | 10/30\n";

        assert_eq!(expected, format_totps(&totps, &march_14_2020));
    }
}
