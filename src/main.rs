#![warn(clippy::all, clippy::pedantic)]

use std::{
    error::Error,
    io::stdout,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use arboard::Clipboard;
use config::load;
use totp::Totp;
use tui::{TotpLineParagraph, TotpListView};

mod config;
mod totp;
mod tui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();

    let clipboard = Arc::new(Mutex::new(
        Clipboard::new().expect("Could not get access to the clipboard."),
    ));

    let totps: Vec<Totp> = load()
        .iter()
        .map(|x| x.clone().expect("Could not parse configuration file."))
        .collect();

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
        1000,
    )?;

    Ok(())
}
