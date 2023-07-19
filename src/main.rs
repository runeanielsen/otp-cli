#![warn(clippy::all, clippy::pedantic)]

use std::{
    env,
    io::stdout,
    path::PathBuf,
    process,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use arboard::Clipboard;
use totp::Totp;
use tui::{TotpLineParagraph, TotpListView};

mod config;
mod totp;
mod tui;

fn main() {
    const INTERVAL: u64 = 30;
    const DIGITS: u32 = 6;
    const POLL_INTERVAL: u64 = 1000;

    // Lets the user specify the path to the TOTP secrets file.
    // In the future we might switch to using a configuration file,
    // but for now it's fine just doing it the simple way.
    // Could also be nice to pass the secrets in using STDIN, that way
    // the user could decrypt their secrets file with their encryption algorithm of choice.
    let args = env::args().collect::<Vec<_>>();
    let config_file_name = if args.len() > 1 {
        args[1].to_string()
    } else {
        "totp.txt".to_string()
    };

    // Getting the home directory works fine on Unix systems,
    // and this project only supports UNIX based systems.
    #[allow(deprecated)]
    let home_dir = env::home_dir().expect("Could not load user home directory.");

    let clipboard = Arc::new(Mutex::new(
        Clipboard::new().expect("Could not get access to the clipboard."),
    ));

    let default_config_path = [home_dir, PathBuf::from(".config/totp-tui")]
        .iter()
        .collect();

    let totps: Vec<Totp> =
        match config::load_totps(default_config_path, &config_file_name, DIGITS, INTERVAL) {
            Ok(result) => result,
            Err(err) => {
                println!("Error: {}", err);
                process::exit(1);
            }
        };

    let mut stdout = stdout();

    match tui::start(
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
    ) {
        Ok(_) => {}
        Err(err) => {
            println!("Error: {}", err);
            process::exit(1);
        }
    };
}
