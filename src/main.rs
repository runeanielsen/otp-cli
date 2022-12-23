#![warn(clippy::all, clippy::pedantic)]

use std::{error::Error, io::stdout};

use config::load;

mod config;
mod totp;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let configs = load();

    ui::start(&mut stdout, &configs)?;

    Ok(())
}
