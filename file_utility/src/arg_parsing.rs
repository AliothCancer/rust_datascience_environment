use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
#[derive(Debug)]
struct Cli {
    /// The CSV file containing the data
    #[arg(short = 'p', long = "data_path", value_name = "data path")]
    data_path: PathBuf,
}

pub fn get_data_path() {
    let args = Cli::parse();

    dbg!(args);
}
