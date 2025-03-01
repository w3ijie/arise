use std::{
    error::Error,
    fs::{self},
};

use clap::Parser;
use cli::Args;
use text::game::Game;

mod cli;
mod models;
mod text;

const DEFAULT_FILE_PATH: &str = "save_state.json";

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let file_path = match args.file.as_str() {
        "" => DEFAULT_FILE_PATH,
        _ => args.file.as_str(),
    };

    let file = fs::metadata(file_path);

    let mut game = match file {
        Ok(_) => Game::load(file_path)?,
        Err(_) => Game::new(file_path),
    };

    game.run()?;

    Ok(())
}
