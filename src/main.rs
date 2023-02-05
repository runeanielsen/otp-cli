#![warn(clippy::all, clippy::pedantic)]

use std::{
    error::Error,
    io::stdout,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use arboard::Clipboard;
use totp::Totp;
use tui::{TotpLineParagraph, TotpListView};

mod config;
mod totp;
mod tui;

fn main() -> Result<(), Box<dyn Error>> {
    let polling_interval = 1000;
    let mut stdout = stdout();

    let clipboard = Arc::new(Mutex::new(
        Clipboard::new().expect("Could not get access to the clipboard."),
    ));

    let totps: Vec<Totp> = config::load_totps()?;

    tui::start(
        &mut stdout,
        vec![
            Box::new(TotpLineParagraph::new()),
            Box::new(TotpListView::new(
                SystemTime::now(),
                totps,
                Arc::clone(&clipboard),
            )),
        ],
        polling_interval,
    )?;

    Ok(())
}
