use std::{error::Error, io::Stdout, time::SystemTime};

use crossterm::{
    cursor, queue,
    style::{self, Stylize},
};

use crate::totp;

use super::{widgets::LineParagraph, Display, Element, HandleEvent, Refresh};

pub struct TotpLineParagraph {
    line_paragraph: LineParagraph,
    redraw: bool,
}

impl TotpLineParagraph {
    pub fn new() -> Self {
        Self {
            line_paragraph: LineParagraph::new(String::new()),
            // It's important to set the redraw to true first time we run,
            // otherwise it will first be redrawn on next change.
            redraw: true,
        }
    }

    fn update_line_text(&mut self, duration_max: u64, time: SystemTime) {
        let duration_used = totp::duration_used(duration_max, time);
        let new_paragraph_text = format!("{duration_used:02}/{duration_max}");

        // No reason to redraw if the text has not changed.
        if new_paragraph_text != self.line_paragraph.text {
            self.line_paragraph.text = new_paragraph_text;
            self.redraw = true;
        }
    }
}

impl Element for TotpLineParagraph {}

impl Display for TotpLineParagraph {
    fn display(&mut self, w: &mut Stdout) -> Result<(), Box<dyn Error>> {
        if self.redraw {
            queue!(
                w,
                style::PrintStyledContent(self.line_paragraph.text.clone().grey()),
            )?;

            self.redraw = false;
        }

        queue!(w, cursor::MoveToNextLine(1))?;

        Ok(())
    }
}

impl Refresh for TotpLineParagraph {
    fn refresh(&mut self) {
        self.update_line_text(30, SystemTime::now());
    }
}

impl HandleEvent for TotpLineParagraph {
    fn handle_event(&mut self, _: &crossterm::event::Event) {}
}
