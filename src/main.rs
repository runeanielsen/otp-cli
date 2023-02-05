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
    const INTERVAL: u64 = 30;
    const DIGITS: u32 = 6;
    const POLL_INTERVAL: u64 = 1000;

    let mut stdout = stdout();

    let clipboard = Arc::new(Mutex::new(
        Clipboard::new().expect("Could not get access to the clipboard."),
    ));

    let totps: Vec<Totp> = config::load_totps(DIGITS, INTERVAL)?;

    tui::start(
        &mut stdout,
        vec![
            Box::new(TotpLineParagraph::new()),
            Box::new(TotpListView::new(
                SystemTime::now(),
                INTERVAL,
                totps,
                Arc::clone(&clipboard),
            )),
        ],
        POLL_INTERVAL,
    )?;

    Ok(())
}
