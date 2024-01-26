use std::env;
use std::path::PathBuf;

fn get_config_root() -> PathBuf {
    PathBuf::from(match env::consts::OS {
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
    })
}

pub fn get_config_folder(name: Option<&str>) -> PathBuf {
    let config_base_path = get_config_root();

    let config_path = match name {
        Some(name) => config_base_path.join(name),
        None => config_base_path,
    };

    return config_path;
}
