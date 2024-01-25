use core::panic;
use serde_json::{from_str, Value};
use std::env;
use std::fs;
use std::path;

fn get_obsidian_folder() -> String {
    match env::consts::OS {
        "linux" => {
            let config_string =
                env::var("HOME").expect("Should be able to find $HOME") + "/.config/obsidian";
            if !path::Path::new(&config_string).exists() {
                let config_string = match env::var("XDG_CONGIG_HOME") {
                    Ok(s) => s + "/obsidian",
                    Err(_) => panic!("Could not find obsidian config folder"),
                };

                if !path::Path::new(&config_string).exists() {
                    panic!("Could not find obsidian config folder");
                }

                return config_string;
            }

            return config_string;
        }
        _ => panic!("Systems is not supported!"),
    }
}

fn main() {
    // System dependant string
    let obsidian_string = get_obsidian_folder();

    // System independant path
    let obsidian_path = path::Path::new(&obsidian_string);

    let file_path = obsidian_path.join("obsidian.json");
    println!("In file {}", file_path.display());

    let contents = fs::read_to_string(file_path).expect("Should be able to read");

    println!("With text:\n{contents}");

    let v: Value = from_str(&contents).expect("Should be able to extract a Value");

    println!("{v}");
}

#[cfg(test)]
mod tests {}
