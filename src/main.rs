use clap::Parser;
use core::{fmt, panic};
use serde_json::{from_str, Value};
use std::path::PathBuf;
use std::{env, fs};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the obsidian config folder, if not specified the folder will be searched in the
    /// system default location.
    #[arg(long)]
    #[arg(value_parser = opt_folder_path)]
    obsidian_path: Option<PathBuf>,

    /// Path to the obsi-tool config folder, if not specified the file will be searched in the system
    /// default location.
    #[arg(short, long)]
    #[arg(value_parser = opt_folder_path)]
    config_path: Option<PathBuf>,
}

#[derive(Debug)]
struct Vault {
    pub id: String,
    pub path: PathBuf,
}

impl fmt::Display for Vault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id:{} path:{}", self.id, self.path.display())
    }
}

fn opt_folder_path(s: &str) -> Result<PathBuf, String> {
    let obsi_path = PathBuf::from(s);
    if obsi_path.exists() {
        if obsi_path.is_dir() {
            Ok(obsi_path)
        } else {
            Err(format!("The specified file isn't a folder"))
        }
    } else {
        Err(format!("The specified folder doesn't exist"))
    }
}

fn get_config_folder(name: Option<&str>) -> PathBuf {
    let config_base_path = PathBuf::from(match env::consts::OS {
        "linux" => match env::var("XDG_CONFIG_HOME") {
            Ok(s) => s,
            Err(_) => env::var("HOME").expect("Couldn't find HOME folder") + "/.config",
        },
        "windows" => env::var("APPDATA").expect("Couldn't find APPDATA folder"),
        "macos" => {
            "/Users/".to_string()
                + &env::var("USER").expect("Couldn't find USER folder")
                + "/Library/Application Support"
        }
        _ => panic!("System is not supported!"),
    });

    let config_path = match name {
        Some(name) => config_base_path.join(name),
        None => config_base_path,
    };

    if !config_path.exists() {
        panic!("Could not find config folder at {}", config_path.display());
    }

    return config_path;
}

fn get_vault_list(obsidian_path: &PathBuf) -> Vec<Vault> {
    let obsidian_manifest_path = obsidian_path.join("obsidian.json");

    if !obsidian_manifest_path.exists() {
        panic!(
            "Cannot find obsidian file at {}",
            obsidian_manifest_path.display()
        );
    }

    if !obsidian_manifest_path.is_file() {
        panic!("{} is not a file", obsidian_manifest_path.display());
    }

    let file_value: Value = from_str(
        &fs::read_to_string(obsidian_manifest_path).expect("Obsidian file should be readable"),
    )
    .expect("Obsidian file should be parseable");

    return file_value["vaults"]
        .as_object()
        .unwrap()
        .into_iter()
        .map(|(id, val)| Vault {
            id: id.to_owned(),
            path: PathBuf::from(val["path"].as_str().unwrap()),
        })
        .collect();
}

fn main() {
    let cli = Cli::parse();

    let obsi_path = cli
        .obsidian_path
        .unwrap_or_else(|| get_config_folder(Some("obsidian")));

    println!("Obsidian config folder: {}", obsi_path.display());

    let config_path = cli.config_path.unwrap_or_else(|| get_config_folder(None)); //Some("obsi-tool")));

    println!("Config folder path: {}", config_path.display());

    let vaults = get_vault_list(&obsi_path);

    vaults.iter().for_each(|v| {
        println!(
            "{}\n Exists: {}; Is dir: {}",
            v,
            v.path.exists(),
            v.path.is_dir()
        )
    });
}
