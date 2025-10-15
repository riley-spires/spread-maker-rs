use clap::Parser;
use std::path::Path;

/// A program to create spread charts locally on your machine and output to image
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// path to csv value containing the x-axis points
    #[arg(short, long)]
    #[arg(default_value_t = String::from("x-axis.csv"))]
    #[arg(value_parser = input_is_valid)]
    pub input: String,
}

fn input_is_valid(input: &str) -> Result<String, String> {
    if !input.ends_with(".csv") {
        return Err(format!("'{}' is not a csv file!", &input));
    }

    let path = Path::new(&input);

    if !path.exists() {
        return Err(format!("'{}' does not exist!", &input));
    }

    Ok(String::from(input))
}
