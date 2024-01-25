use clap::Parser;
use core::panic;
use serde_json::{from_str, Value};
use std::env;
use std::fs;
use std::path;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the obsidian config folder, if not specified the folder will be searched in the
    /// default location.
    #[arg(short, long)]
    config_path: Option<String>,
}

fn get_obsidian_folder() -> PathBuf {
    let config_string = match env::consts::OS {
        "linux" => {
            let mut config_string =
                env::var("HOME").expect("Couldn't find HOME folder") + "/.config/obsidian";
            if !path::Path::new(&config_string).exists() {
                config_string = match env::var("XDG_CONGIG_HOME") {
                    Ok(s) => s + "/obsidian",
                    Err(_) => panic!("Could not find obsidian config folder"),
                };
            }
            config_string
        }
        "windows" => env::var("APPDATA").expect("Couldn't find APPDATA folder") + "\\obsidian",
        "macos" => {
            "/Users/".to_string()
                + &env::var("USER").expect("Couldn't find USER folder")
                + "/Library/Application Support/obsidian"
        }
        _ => panic!("System is not supported!"),
    };

    let config_path = PathBuf::from(config_string);

    if !config_path.exists() {
        panic!("Could not find obsidian config folder");
    }

    return config_path;
}

fn main() {
    let cli = Cli::parse();

    let obsi_path = match cli.config_path {
        Some(s) => PathBuf::from(s),
        None => get_obsidian_folder(),
    };

    println!("Path: {:?}", obsi_path);
}

#[cfg(test)]
mod tests {}
