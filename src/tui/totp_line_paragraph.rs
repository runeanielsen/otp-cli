use std::{io::Stdout, time::SystemTime};

use crossterm::{
    cursor, queue,
    style::{self, Stylize},
};

use crate::totp;

use super::{widgets::LineParagraph, Display, Element, HandleEvent, Refresh};

pub struct TotpLineParagraph {
    line_paragraph: LineParagraph,
}

impl TotpLineParagraph {
    pub fn new() -> Self {
        Self {
            line_paragraph: LineParagraph::new(String::new()),
        }
    }
}

impl Element for TotpLineParagraph {}

impl Display for TotpLineParagraph {
    fn display(&mut self, w: &mut Stdout) {
        queue!(
            w,
            style::PrintStyledContent(self.line_paragraph.text.clone().grey()),
            cursor::MoveToNextLine(1)
        )
        .unwrap();
    }
}

impl Refresh for TotpLineParagraph {
    fn refresh(&mut self) {
        let interval = 30;
        let duration_used = totp::duration_used(interval, SystemTime::now());
        self.line_paragraph.text = format!("{duration_used:02}/{interval}");
    }
}

impl HandleEvent for TotpLineParagraph {
    fn handle_event(&mut self, _: &crossterm::event::Event) {}
}
