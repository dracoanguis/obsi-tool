use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the obsidian config folder, if not specified the folder will be searched in the
    /// system default location.
    #[arg(long)]
    #[arg(value_parser = opt_folder_path)]
    pub obsidian_path: Option<PathBuf>,

    /// Path to the obsi-tool config folder, if not specified the file will be searched in the system
    /// default location.
    #[arg(short, long)]
    #[arg(value_parser = opt_folder_path)]
    pub config_path: Option<PathBuf>,
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
