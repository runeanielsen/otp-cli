use std::{
    error::Error,
    io::Write,
    time::{Duration, SystemTime},
};

use crossterm::{
    cursor,
    event::{self, poll, Event, KeyCode},
    execute, queue,
    style::{self, Stylize},
    terminal::{self, ClearType},
};

use crate::totp::Totp;

fn longest_name_char_count(configs: &[Totp]) -> Option<usize> {
    configs
        .iter()
        .max_by(|x, y| x.name.len().cmp(&y.name.len()))
        .map(|config| config.name.len())
}

fn format_totp(config: &Totp, time: &SystemTime, name_max_length: usize) -> String {
    format!(
        "{:<max_length$} | {:0digits_width$} | {:02}/{}",
        config.name,
        config.code(time),
        config.duration_used(time),
        config.interval,
        digits_width = config.digits as usize,
        max_length = name_max_length
    )
}

pub fn start<W>(w: &mut W, configs: &[Totp]) -> Result<(), Box<dyn Error>>
where
    W: Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let name_max_length = longest_name_char_count(configs).unwrap();

    // We use the line count starting for 0, so we want to substract a single value.
    let max_index_count = configs.len().saturating_sub(1);
    let mut current_index = 0;

    loop {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        let now = SystemTime::now();
        for (index, line) in configs
            .iter()
            .map(|x| format_totp(x, &now, name_max_length))
            .enumerate()
        {
            if index == current_index {
                queue!(
                    w,
                    style::PrintStyledContent(line.magenta()),
                    cursor::MoveToNextLine(1)
                )?;
            } else {
                queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
            }
        }

        w.flush()?;

        if poll(Duration::from_millis(250))? {
            let event = event::read()?;

            if event == Event::Key(KeyCode::Char('j').into()) {
                if current_index < max_index_count {
                    current_index = current_index.saturating_add(1);
                }
            } else if event == Event::Key(KeyCode::Char('k').into()) {
                current_index = current_index.saturating_sub(1);
            } else if event == Event::Key(KeyCode::Char('q').into()) {
                break;
            }
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
        let assertions = [
            (
                "Acme Inc.         | 640572 | 10/30",
                Totp::new("Acme Inc.", "8n4mzt7w", 6, 30),
            ),
            (
                "Gizmo Corporation | 087439 | 10/30",
                Totp::new("Gizmo Corporation", "xkc2j8fh", 6, 30),
            ),
            (
                "Foo Industries    | 771990 | 10/30",
                Totp::new("Foo Industries", "9s6bk3jq", 6, 30),
            ),
        ];

        for (expected, input) in assertions {
            assert_eq!(expected, format_totp(&input, &march_14_2020, 17));
        }
    }
}
