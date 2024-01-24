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
                let config_string = 
            }

            return config_string;
        }
        _ => panic!("Systems is not supported!"),
    }
}

fn main() {
    let file_path = "/home/gargan/.config/obsidian/obsidian.json";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should be able to read");

    println!("With text:\n{contents}");

    let v: Value = from_str(&contents).expect("Should be able to extract a Value");

    println!("{v}");
}

#[cfg(test)]
mod tests {}
