mod util;
use clap::Parser;
use util::cli::Cli;
use util::obsidian::*;
use util::util::*;

fn main() {
    let cli = Cli::parse();

    let obsi_path = cli
        .obsidian_path
        .unwrap_or_else(|| get_config_folder(Some("obsidian")));

    println!("Obsidian config folder: {}", obsi_path.display());

    let config_path = cli
        .config_path
        .unwrap_or_else(|| get_config_folder(Some("obsi-tool")));

    println!("Config folder path: {}", config_path.display());

    let vaults = get_vault_list(&obsi_path);

    vaults
        .iter()
        .for_each(|v| println!("{}\n Is valid: {}", v, v.check_validity()));

    let mappings = vaults[5].get_mappings();

    mappings.iter().for_each(|m| println!("{:?}", m));
}
