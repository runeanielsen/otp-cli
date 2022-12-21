#![warn(clippy::all, clippy::pedantic)]

use std::{
    error::Error,
    io::{stdout, Write},
    time::{Duration, SystemTime},
};

use crossterm::{
    cursor,
    event::{self, poll, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
};

mod otp;

#[derive(Clone)]
struct OtpConfig {
    digits: u32,
    interval: u64,
    name: String,
    secret: String,
}

fn load_otp_config() -> Vec<OtpConfig> {
    vec![
        OtpConfig {
            name: "Mastodon".to_string(),
            secret: "hello".to_string(),
            interval: 30,
            digits: 6,
        },
        OtpConfig {
            name: "Codeberg".to_string(),
            secret: "hello world!".to_string(),
            interval: 30,
            digits: 6,
        },
        OtpConfig {
            name: "Codeberg 1".to_string(),
            secret: "hello world!1".to_string(),
            interval: 30,
            digits: 6,
        },
        OtpConfig {
            name: "Codeberg 2".to_string(),
            secret: "hello world!2".to_string(),
            interval: 60,
            digits: 6,
        },
        OtpConfig {
            name: "Codeberg 3".to_string(),
            secret: "hello world!3".to_string(),
            interval: 30,
            digits: 8,
        },
    ]
}

fn step_counter(time: &SystemTime, step: u64) -> u64 {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % step
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

fn longest_config_name_by_count(configs: &[OtpConfig]) -> usize {
    let mut longest_char_count = 0;
    for config in configs {
        let char_count = config.name.len();
        if char_count > longest_char_count {
            longest_char_count = char_count;
        }
    }

    longest_char_count
}

fn most_otp_digits(configs: &[OtpConfig]) -> u32 {
    let mut most_digits = 0;
    for config in configs {
        let digits = config.digits;
        if digits > most_digits {
            most_digits = digits;
        }
    }

    most_digits
}

fn otp_display(configs: &[OtpConfig], time: &SystemTime) -> String {
    configs
        .iter()
        .map(|x| {
            format!(
                "{:<max_length$} | {:<digits_width$} | {:02}/{}\n",
                x.name,
                format!(
                    "{:0width$}",
                    otp::totp(&x.secret, x.interval, x.digits),
                    width = x.digits as usize
                ),
                step_counter(time, x.interval),
                x.interval,
                digits_width = most_otp_digits(configs) as usize,
                max_length = longest_config_name_by_count(configs)
            )
        })
        .collect::<String>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();

    execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let configs = load_otp_config();

    loop {
        queue!(
            stdout,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        for line in otp_display(&configs, &SystemTime::now()).split('\n') {
            queue!(stdout, style::Print(line), cursor::MoveToNextLine(1))?;
        }

        stdout.flush()?;

        if poll(Duration::from_millis(1_000))? {
            if let 'q' = read_char().expect("Could not read input.") {
                break;
            };
        }
    }

    execute!(
        stdout,
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
    fn can_find_longest_config_name_by_count() {
        let configs = [
            OtpConfig {
                name: "Mastodon".to_string(),
                secret: "hello".to_string(),
                interval: 30,
                digits: 6,
            },
            OtpConfig {
                name: "Codeberg".to_string(),
                secret: "hello world!".to_string(),
                interval: 30,
                digits: 6,
            },
            OtpConfig {
                name: "Codeberg 1".to_string(),
                secret: "hello world!1".to_string(),
                interval: 30,
                digits: 6,
            },
            OtpConfig {
                name: "This is a very long name".to_string(),
                secret: "hello world!1".to_string(),
                interval: 30,
                digits: 6,
            },
            OtpConfig {
                name: "Codeberg 2".to_string(),
                secret: "hello world!2".to_string(),
                interval: 60,
                digits: 6,
            },
            OtpConfig {
                name: "Codeberg 3".to_string(),
                secret: "hello world!3".to_string(),
                interval: 30,
                digits: 8,
            },
        ];

        assert_eq!(24, longest_config_name_by_count(&configs));
    }

    #[test]
    fn can_find_most_otp_digits_in_configs() {
        let configs = [
            OtpConfig {
                name: "Mastodon".to_string(),
                secret: "hello".to_string(),
                interval: 30,
                digits: 6,
            },
            OtpConfig {
                name: "Codeberg".to_string(),
                secret: "hello world!".to_string(),
                interval: 30,
                digits: 6,
            },
            OtpConfig {
                name: "Codeberg 1".to_string(),
                secret: "hello world!1".to_string(),
                interval: 30,
                digits: 6,
            },
            OtpConfig {
                name: "This is a very long name".to_string(),
                secret: "hello world!1".to_string(),
                interval: 30,
                digits: 6,
            },
            OtpConfig {
                name: "Codeberg 2".to_string(),
                secret: "hello world!2".to_string(),
                interval: 60,
                digits: 6,
            },
            OtpConfig {
                name: "Codeberg 3".to_string(),
                secret: "hello world!3".to_string(),
                interval: 30,
                digits: 8,
            },
        ];

        assert_eq!(8, most_otp_digits(&configs));
    }
}
