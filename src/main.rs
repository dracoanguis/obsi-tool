use core::panic;
use serde_json::{from_str, Value};
use std::env;
use std::fs;
use std::path;
use std::path::PathBuf;

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
    let obsidian_path = get_obsidian_folder();

    let file_path = obsidian_path.join("obsidian.json");
    println!("In file {}", file_path.display());

    let contents = fs::read_to_string(file_path).expect("Should be able to read");

    println!("With text:\n{contents}");

    let v: Value = from_str(&contents).expect("Should be able to extract a Value");

    println!("{v}");
}

#[cfg(test)]
mod tests {}
