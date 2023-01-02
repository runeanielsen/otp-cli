use std::{
    error::Error,
    io::Write,
    time::{Duration, SystemTime},
};

use arboard::Clipboard;
use crossterm::{
    cursor,
    event::{self, poll, Event, KeyCode},
    execute, queue,
    style::{self, Stylize},
    terminal,
};

use crate::totp::{self, Totp};

fn longest_name_char_count(configs: &[Totp]) -> Option<usize> {
    configs
        .iter()
        .max_by(|x, y| x.name.len().cmp(&y.name.len()))
        .map(|config| config.name.len())
}

fn format_totp(config: &Totp, time: SystemTime, name_max_length: usize) -> String {
    format!(
        "{:<max_length$} | {:0digits_width$}",
        config.name,
        config.code(time),
        digits_width = config.digits as usize,
        max_length = name_max_length
    )
}

struct State {
    max_index: usize,
    current_index: usize,
    lines: Vec<Line>,
}

impl State {
    fn new(lines: Vec<Line>, max_index: usize) -> Self {
        State {
            max_index,
            current_index: 0,
            lines,
        }
    }
}

struct Line {
    text: String,
    modified: bool,
}

impl Line {
    fn new(text: String, modified: bool) -> Self {
        Line { text, modified }
    }
}

pub fn start<W>(w: &mut W, configs: &[Totp]) -> Result<(), Box<dyn Error>>
where
    W: Write,
{
    let mut clipboard = Clipboard::new().unwrap();

    execute!(w, terminal::EnterAlternateScreen, cursor::Hide)?;
    terminal::enable_raw_mode()?;

    let name_max_length = longest_name_char_count(configs).unwrap();
    let interval = 30;

    // We use the line count starting for 0, so we want to substract a single value.
    let mut state = State::new(
        configs
            .iter()
            .map(|x| Line::new(format_totp(x, SystemTime::now(), name_max_length), true))
            .collect(),
        configs.len().saturating_sub(1),
    );

    loop {
        let now = SystemTime::now();
        let duration_used = totp::duration_used(interval, now);

        let duration_line =
            style::PrintStyledContent(format!("{duration_used:02}/{interval}").green());

        queue!(
            w,
            cursor::MoveTo(0, 0),
            duration_line,
            cursor::MoveToNextLine(1)
        )?;

        if duration_used == 0 {
            for (index, config) in configs.iter().enumerate() {
                let mut line = &mut state.lines[index];
                line.text = format_totp(config, now, name_max_length);
                line.modified = true;
            }
        }

        for index in 0..state.lines.len() {
            let mut line = &mut state.lines[index];
            if line.modified {
                if index == state.current_index {
                    queue!(w, style::PrintStyledContent(line.text.clone().blue()))?;
                } else {
                    queue!(w, style::Print(line.text.clone()))?;
                };

                line.modified = false;
            }

            queue!(w, cursor::MoveToNextLine(1))?;
        }

        w.flush()?;

        if poll(Duration::from_millis(500))? {
            let event = event::read()?;

            if event == Event::Key(KeyCode::Char('j').into()) {
                if state.current_index < state.max_index {
                    // Old selected is modified, it is no longer selected
                    state.lines[state.current_index].modified = true;

                    state.current_index = state.current_index.saturating_add(1);

                    // New selection is modified, it is now selected
                    state.lines[state.current_index].modified = true;
                }
            } else if event == Event::Key(KeyCode::Char('k').into()) {
                // Old selected is modified, it is no longer selected
                state.lines[state.current_index].modified = true;

                state.current_index = state.current_index.saturating_sub(1);

                // New selection is modified, it is now selected
                state.lines[state.current_index].modified = true;
            } else if event == Event::Key(KeyCode::Char('q').into()) {
                break;
            } else if event == Event::Key(KeyCode::Enter.into()) {
                let currently_selected = &configs[state.current_index];
                let code = format!(
                    "{:0digits$}",
                    currently_selected.code(now),
                    digits = currently_selected.digits as usize
                );
                clipboard
                    .set_text(code)
                    .expect("Could not copy to clipboard.");
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
            assert_eq!(expected, format_totp(&input, march_14_2020, 17));
        }
    }
}
