use arboard::Clipboard;
use crossterm::style::Stylize;
use crossterm::{
    cursor,
    event::{Event, KeyCode},
    queue, style,
};
use std::error::Error;
use std::io::Stdout;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use crate::totp::{self, Totp};

use super::widgets::{LineItem, ListView};
use super::{Display, Element, HandleEvent, Refresh};

pub struct TotpListView {
    totps: Vec<Totp>,
    list_view: ListView<Totp>,
    interval: u64,
}

fn format_totp(config: &Totp, time: SystemTime, name_max_length: usize) -> String {
    format!(
        "{:<max_length$} | {}",
        config.name,
        config.code_padded(time),
        max_length = name_max_length
    )
}

fn create_line_items(totps: &[Totp], time: SystemTime) -> Vec<LineItem<Totp>> {
    let max_name_length = totps
        .iter()
        .max_by(|x, y| x.name.chars().count().cmp(&y.name.chars().count()))
        .map(|config| config.name.chars().count())
        .expect("Could not get the longest name count.");

    totps
        .iter()
        .map(|config| LineItem::new(&format_totp(config, time, max_name_length), config.clone()))
        .collect()
}

impl TotpListView {
    pub fn new(
        time: SystemTime,
        interval: u64,
        totps: Vec<Totp>,
        clipboard: Arc<Mutex<Clipboard>>,
    ) -> Self {
        let line_items = create_line_items(&totps, time);
        Self {
            totps,
            list_view: ListView::new(
                line_items,
                Box::new(move |totp| {
                    clipboard
                        .lock()
                        .expect("Could not lock clipboard.")
                        .set_text(totp.code_padded(time))
                        .expect("Could not set text in clipboard.");
                }),
            ),
            interval,
        }
    }
}

impl Element for TotpListView {}

impl Display for TotpListView {
    fn display(&mut self, w: &mut Stdout) -> Result<(), Box<dyn Error>> {
        for (index, line) in self.list_view.line_items.iter_mut().enumerate() {
            if line.modified {
                if index == self.list_view.current_index {
                    queue!(w, style::PrintStyledContent(line.text.clone().blue()))?;
                } else {
                    queue!(w, style::Print(line.text.clone()))?;
                };

                // After we have drawed it, we set it to no longer modified
                // because we do not want to draw it again.
                line.modified = false;
            }

            queue!(w, cursor::MoveToNextLine(1))?;
        }

        Ok(())
    }
}

impl HandleEvent for TotpListView {
    fn handle_event(&mut self, event: &Event) {
        if event == &Event::Key(KeyCode::Char('j').into()) {
            self.list_view.select_next();
        } else if event == &Event::Key(KeyCode::Char('k').into()) {
            self.list_view.select_prev();
        } else if event == &Event::Key(KeyCode::Enter.into()) {
            (self.list_view.selected_callback)(
                &self.list_view.line_items[self.list_view.current_index].value,
            );
        }
    }
}

impl Refresh for TotpListView {
    fn refresh(&mut self) {
        let now = SystemTime::now();
        let duration_used = totp::duration_used(self.interval, now);
        if duration_used == 0 {
            self.list_view
                .set_line_items(create_line_items(&self.totps, now));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn totp_is_formatted_correctly() {
        let interval = 30;
        let digits = 6;

        let march_14_2020 = SystemTime::UNIX_EPOCH + Duration::new(1_584_188_800, 0);
        let assertions = [
            (
                "Acme Inc.         | 640572",
                Totp::new("Acme Inc.", "8n4mzt7w", digits, interval),
            ),
            (
                "Gizmo Corporation | 087439",
                Totp::new("Gizmo Corporation", "xkc2j8fh", digits, interval),
            ),
            (
                "Foo Industries    | 771990",
                Totp::new("Foo Industries", "9s6bk3jq", digits, interval),
            ),
        ];

        let name_max_length = 17;
        for (expected, input) in assertions {
            assert_eq!(
                expected,
                format_totp(&input, march_14_2020, name_max_length)
            );
        }
    }
}
