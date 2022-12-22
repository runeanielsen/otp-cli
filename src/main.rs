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

use config::{load, longest_name_char_count, max_digits, Config};

mod config;
mod otp;

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

fn otp_display(configs: &[Config], time: &SystemTime) -> String {
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
                digits_width = max_digits(configs).unwrap() as usize,
                max_length = longest_name_char_count(configs).unwrap()
            )
        })
        .collect::<String>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let configs = load();

    let mut stdout = stdout();

    execute!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

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
