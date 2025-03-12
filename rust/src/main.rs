use std::{
    error::Error,
    fs::{self},
};

use clap::Parser;
use cli::Args;

mod cli;
mod models;
mod text;
mod tui;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let file_path = args.file.as_str();

    let file = fs::metadata(file_path);

    if args.text {
        let mut game = match file {
            Ok(_) => text::game::Game::load(file_path)?,
            Err(_) => text::game::Game::new(file_path),
        };

        game.run()?;
    } else {
        let mut terminal = ratatui::init();
        let mut game = match file {
            Ok(_) => tui::game::Game::load(file_path)?,
            Err(_) => tui::game::Game::new(file_path),
        };
        let _ = game.run(&mut terminal);
        ratatui::restore();
    }

    Ok(())
}
