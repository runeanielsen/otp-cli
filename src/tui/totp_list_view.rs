use arboard::Clipboard;
use crossterm::event::KeyEvent;
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType};
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

enum TotpCommandType {
    Search,
}

struct TotpCommand {
    command_type: TotpCommandType,
    input: String,
}

impl TotpCommand {
    fn new(command_type: TotpCommandType) -> Self {
        Self {
            command_type,
            input: String::new(),
        }
    }
}

pub struct TotpListView {
    totps: Vec<Totp>,
    list_view: ListView<Totp>,
    interval: u64,
    // Refreshed flag is there to avoid multiple refreshes in a row when duration is 0.
    refreshed: bool,
    command: Option<TotpCommand>,
}

fn format_totp(config: &Totp, time: SystemTime, name_max_length: usize) -> String {
    format!(
        "{:<max_length$} | {}",
        config.name,
        config.code_padded(time),
        max_length = name_max_length
    )
}

fn longest_string(strs: &[&str]) -> Option<usize> {
    strs.iter()
        .max_by(|x, y| x.chars().count().cmp(&y.chars().count()))
        .map(|config| config.chars().count())
}

fn create_line_items(totps: &[Totp], time: SystemTime) -> Vec<LineItem<Totp>> {
    let names: Vec<&str> = totps.iter().map(|x| x.name.as_str()).collect();
    let max_name_length = longest_string(&names).expect("Could not get longest name.");

    totps
        .iter()
        .map(|config| LineItem::new(&format_totp(config, time, max_name_length), config.clone()))
        .collect()
}

fn find_line_item_matching_search(
    line_items: &[LineItem<Totp>],
    search_text: &str,
) -> Option<usize> {
    let mut found_index: Option<usize> = None;
    for (index, line_item) in line_items.iter().enumerate() {
        if line_item
            .text()
            .to_lowercase()
            .starts_with(&search_text.to_lowercase())
        {
            found_index = Some(index);
        }
    }

    found_index
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
                    let time = SystemTime::now();
                    clipboard
                        .lock()
                        .expect("Could not lock clipboard.")
                        .set_text(totp.code_padded(time))
                        .expect("Could not set text in clipboard.");
                }),
            ),
            interval,
            refreshed: false,
            command: None,
        }
    }
}

impl Element for TotpListView {}

impl Display for TotpListView {
    fn display(&mut self, w: &mut Stdout) -> Result<(), Box<dyn Error>> {
        queue!(w, Clear(ClearType::FromCursorDown))?;

        for (index, line) in self.list_view.line_items.iter_mut().enumerate() {
            if index == self.list_view.current_index {
                queue!(w, style::PrintStyledContent(line.text().blue()))?;
            } else {
                queue!(w, style::Print(line.text()))?;
            };

            if line.is_marked() {
                // When the line is marked we want to indicate it with a '*'.
                queue!(w, style::Print(" *".blue()))?;
            }

            queue!(w, cursor::MoveToNextLine(1))?;
        }

        match &self.command {
            Some(command) => {
                queue!(w, cursor::MoveToNextLine(1))?;
                queue!(w, style::Print("/".to_string() + &command.input))?;
            }
            None => {}
        }

        Ok(())
    }
}

impl HandleEvent for TotpListView {
    fn handle_event(&mut self, event: &Event) {
        match &mut self.command {
            Some(command) => match command.command_type {
                TotpCommandType::Search => {
                    if event == &Event::Key(KeyCode::Backspace.into()) {
                        command.input.pop();
                    } else if event == &Event::Key(KeyCode::Enter.into())
                        || event == &Event::Key(KeyCode::Esc.into())
                    {
                        self.command = None;
                    } else if let Event::Key(KeyEvent {
                        code: KeyCode::Char(char),
                        modifiers: _,
                        kind: _,
                        state: _,
                    }) = event
                    {
                        command.input.push(*char);
                        if let Some(match_index) = find_line_item_matching_search(
                            &self.list_view.line_items,
                            &command.input,
                        ) {
                            self.list_view.current_index = match_index;
                        };
                    }
                }
            },
            _ => {
                if event == &Event::Key(KeyCode::Char('j').into()) {
                    self.list_view.select_next();
                } else if event == &Event::Key(KeyCode::Char('k').into()) {
                    self.list_view.select_prev();
                } else if event == &Event::Key(KeyCode::Enter.into()) {
                    self.list_view.mark_selected_line_item();
                    (self.list_view.selected_callback)(
                        self.list_view.get_selected_line_item().value(),
                    );
                } else if event == &Event::Key(KeyCode::Char('/').into()) {
                    self.command = Some(TotpCommand::new(TotpCommandType::Search));
                }
            }
        }
    }
}

impl Refresh for TotpListView {
    fn refresh(&mut self) {
        let now = SystemTime::now();
        let duration_used = totp::duration_used(self.interval, now);
        // The refreshed flag is set to avoid an issue where on duration 0,
        // the view is refreshed multiple times when interacting with the TUI.
        if !self.refreshed && duration_used == 0 {
            self.list_view
                .set_line_items(create_line_items(&self.totps, now));
            self.refreshed = true;
        } else if self.refreshed {
            self.refreshed = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn find_longest_string() {
        let assertions = [
            (vec!["Hello", "Hello World!", "Wow!"], Some(12)),
            (vec!["Hello", "Wow!"], Some(5)),
            (vec!["", "Wow!"], Some(4)),
            (vec!["", ""], Some(0)),
            (vec![], None),
        ];

        for (input, expected) in assertions {
            assert_eq!(longest_string(&input), expected);
        }
    }

    #[test]
    fn totp_is_formatted_correctly() {
        let interval = 30;
        let digits = 6;

        let march_14_2020 = SystemTime::UNIX_EPOCH + Duration::new(1_584_188_800, 0);
        let assertions = [
            (
                "Acme Inc.         | 470961",
                Totp::new("Acme Inc.", "ZDPCQ3LXMFRGIZTDONSWMZDF", digits, interval),
            ),
            (
                "Gizmo Corporation | 783941",
                Totp::new("Gizmo Corporation", "54OMZJQ=", digits, interval),
            ),
            (
                "Foo Industries    | 022136",
                Totp::new("Foo Industries", "32XWIZLOMUVZWK4Q=", digits, interval),
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
