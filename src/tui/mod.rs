mod totp_line_paragraph;
mod totp_list_view;
mod widgets;

pub use totp_line_paragraph::TotpLineParagraph;
pub use totp_list_view::TotpListView;

use std::{
    error::Error,
    io::{Stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, poll, Event, KeyCode},
    execute, queue, style, terminal,
};

pub trait Display {
    fn display(&mut self, w: &mut Stdout) -> Result<(), Box<dyn Error>>;
}

pub trait HandleEvent {
    fn handle_event(&mut self, event: &Event);
}

pub trait Refresh {
    fn refresh(&mut self);
}

pub trait Element: Display + Refresh + HandleEvent {}

pub fn start(
    w: &mut Stdout,
    mut elements: Vec<Box<dyn Element>>,
    pool_interval: u64,
) -> Result<(), Box<dyn Error>> {
    execute!(w, terminal::EnterAlternateScreen, cursor::Hide)?;
    terminal::enable_raw_mode()?;

    'main: loop {
        queue!(w, cursor::MoveTo(0, 0),)?;

        for element in &mut elements {
            element.refresh();
            element.display(w)?;

            w.flush()?;
        }

        if poll(Duration::from_millis(pool_interval))? {
            let event = event::read()?;

            for element in &mut elements {
                element.handle_event(&event);
            }

            if event == Event::Key(KeyCode::Char('q').into()) {
                break 'main;
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
