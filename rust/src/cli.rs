use clap::Parser;

const DEFAULT_FILE_PATH: &str = "save_state.json";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The save file to use
    #[arg(short, long, default_value_t={DEFAULT_FILE_PATH.to_string()})]
    pub file: String,

    /// To play the text-based version
    #[arg(short, long, default_value_t = false)]
    pub text: bool,
}
