use std::{
    error::Error,
    io::Write,
    sync::{Arc, Mutex},
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

use crate::{
    list_view::{LineItem, ListView},
    totp::{self, Totp},
};

fn longest_name_char_count(configs: &[Totp]) -> Option<usize> {
    configs
        .iter()
        .max_by(|x, y| x.name.chars().count().cmp(&y.name.chars().count()))
        .map(|config| config.name.chars().count())
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

pub fn start<W>(
    w: &mut W,
    clipboard: Arc<Mutex<Clipboard>>,
    configs: &[Totp],
) -> Result<(), Box<dyn Error>>
where
    W: Write,
{
    execute!(w, terminal::EnterAlternateScreen, cursor::Hide)?;
    terminal::enable_raw_mode()?;

    let name_max_length = longest_name_char_count(configs).unwrap();
    let interval = 30;

    let create_line_items = |now: SystemTime| -> Vec<LineItem> {
        configs
            .iter()
            .map(|config| LineItem::new(&format_totp(config, now, name_max_length)))
            .collect()
    };

    let mut list_view = ListView::new(
        create_line_items(SystemTime::now()),
        Box::new(move |text| {
            clipboard
                .lock()
                .unwrap()
                .set_text(text)
                .expect("Could not set text in clipboard.");
        }),
    );

    loop {
        let now = SystemTime::now();
        let duration_used = totp::duration_used(interval, now);

        let duration_line =
            style::PrintStyledContent(format!("{duration_used:02}/{interval}").grey());

        queue!(
            w,
            cursor::MoveTo(0, 0),
            duration_line,
            cursor::MoveToNextLine(1)
        )?;

        if duration_used == 0 {
            list_view.set_line_items(create_line_items(now));
        }

        list_view.display(w);

        w.flush()?;

        if poll(Duration::from_millis(1000))? {
            let event = event::read()?;

            list_view.handle_event(&event);

            if event == Event::Key(KeyCode::Char('q').into()) {
                break;
            }
        }
    }

    // Cleanup and reset the terminal when the user quits.
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
                "Acme Inc.         | 640572",
                Totp::new("Acme Inc.", "8n4mzt7w", 6, 30),
            ),
            (
                "Gizmo Corporation | 087439",
                Totp::new("Gizmo Corporation", "xkc2j8fh", 6, 30),
            ),
            (
                "Foo Industries    | 771990",
                Totp::new("Foo Industries", "9s6bk3jq", 6, 30),
            ),
        ];

        for (expected, input) in assertions {
            assert_eq!(expected, format_totp(&input, march_14_2020, 17));
        }
    }
}
