#![warn(clippy::all, clippy::pedantic)]

use std::{
    error::Error,
    io::stdout,
    sync::{Arc, Mutex},
};

use arboard::Clipboard;
use config::load;
use totp::Totp;

mod config;
mod list_view;
mod totp;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();

    let clipboard = Arc::new(Mutex::new(
        Clipboard::new().expect("Could not get access to the clipboard."),
    ));

    let configs: Vec<Totp> = load()
        .iter()
        .map(|x| x.clone().expect("Could not parse configuration file."))
        .collect();

    ui::start(&mut stdout, Arc::clone(&clipboard), &configs)?;

    Ok(())
}
