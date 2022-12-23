#![warn(clippy::all, clippy::pedantic)]

use std::{error::Error, io::stdout};

use config::load;
use totp::Totp;

mod config;
mod totp;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let configs: Vec<Totp> = load()
        .iter()
        .map(|x| x.clone().expect("Could not parse configuration file."))
        .collect();

    ui::start(&mut stdout, &configs)?;

    Ok(())
}
