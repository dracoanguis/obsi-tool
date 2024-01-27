use serde_json::{from_str, Value};
use std::path::PathBuf;
use std::{fmt, fs};

#[derive(Debug)]
pub struct KeySeq {
    pub modifiers: Vec<String>,
    pub key: String,
}

impl KeySeq {
    pub fn from_value(value: &Value) -> Option<KeySeq> {
        let ob = match value.as_object() {
            Some(o) => o,
            None => return None,
        };
        let modifiers: Vec<String> = match ob["modifiers"].as_array() {
            Some(a) => a.into_iter().map(|v| v.to_string()).collect(),
            None => return None,
        };
        let key: String = ob["key"].to_string();
        return Some(KeySeq { modifiers, key });
    }
}

#[derive(Debug)]
pub struct Mapping {
    pub target: String,
    pub maps: Vec<KeySeq>,
}

impl Mapping {
    pub fn from_target_value(target: &String, value: &Value) -> Option<Mapping> {
        let arr = match value.as_array() {
            Some(a) => a,
            None => return None,
        };
        let maps = arr
            .into_iter()
            .map(|v| KeySeq::from_value(v))
            .filter(|oks| matches!(oks, Some(_)))
            .map(|ks| ks.unwrap())
            .collect();
        Some(Mapping {
            target: target.to_string(),
            maps,
        })
    }
}

#[derive(Debug)]
pub struct Vault {
    pub id: String,
    pub path: PathBuf,
}

impl Vault {
    pub fn check_validity(&self) -> bool {
        self.path.exists()
            && self.path.is_dir()
            && self.path.join(".obsidian").exists()
            && self.path.join(".obsidian").is_dir()
    }

    pub fn get_mappings(&self) -> Option<Vec<Mapping>> {
        let hot_path = self.path.join(".obsidian").join("hotkeys.json");
        if !hot_path.exists() {
            return None;
        }
        Some(
            match from_str::<Value>(
                &fs::read_to_string(hot_path).expect("Should be able to read hotkeys if exists"),
            )
            .expect("Should be parseable if readable")
            .as_object()
            {
                Some(o) => o
                    .into_iter()
                    .map(|(s, v)| Mapping::from_target_value(s, v))
                    .filter(|om| matches!(om, Some(_)))
                    .map(|m| m.unwrap())
                    .collect(),
                None => return None,
            },
        )
    }
}

impl fmt::Display for Vault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id:{} path:{}", self.id, self.path.display())
    }
}

pub fn get_vault_list(obsidian_path: &PathBuf) -> Vec<Vault> {
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
