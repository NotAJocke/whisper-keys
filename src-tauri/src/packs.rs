use anyhow::anyhow;
use kira::sound::static_sound::StaticSoundData;
use rayon::prelude::*;
use std::fs;
use std::{
    collections::HashMap,
    ffi::OsString,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct PacksConfig {
    pub creator: String,
    pub source: String,
    pub default_volume: String,
    pub keys: HashMap<String, String>,
}

pub struct Pack {
    pub name: String,
    pub volume: u16,
    pub keys: HashMap<String, StaticSoundData>,
}

pub fn list_available(path: &PathBuf) -> anyhow::Result<Vec<String>> {
    let items = fs::read_dir(path)?;

    let subdirs: Vec<OsString> = items
        .filter_map(|d| {
            let entry = d.ok()?;
            let path = entry.path();
            if path.is_dir() {
                Some(entry.file_name())
            } else {
                None
            }
        })
        .collect();

    let mut packs: Vec<String> = Vec::new();
    for dir in &subdirs {
        let path = Path::new(&path).join(dir);
        let files = fs::read_dir(&path).unwrap();
        let filesnames = files
            .filter_map(|f| {
                let entry = f.ok()?;
                let path = entry.path();
                if path.is_file() {
                    Some(entry.file_name())
                } else {
                    None
                }
            })
            .collect::<Vec<OsString>>();
        let has_config_file = filesnames.contains(&OsString::from("config.json"))
            || filesnames.contains(&OsString::from("config.json5"));
        if has_config_file {
            packs.push(dir.to_str().unwrap().to_owned());
        }
    }

    Ok(packs)
}

pub fn load_pack(folder: &PathBuf, pack_name: &str) -> anyhow::Result<Pack> {
    let path = Path::new(&folder).join(pack_name);
    let config = match fs::read_to_string(path.join("config.json5")) {
        Ok(config) => config,
        Err(_) => fs::read_to_string(path.join("config.json"))?,
    };
    let parsed_config: PacksConfig =
        json5::from_str(&config).map_err(|e| anyhow!("Invalid configuration file: {e}"))?;

    let pack_keys = parsed_config
        .keys
        .par_iter()
        .map(|(key, value)| {
            let filepath = path.join(value);

            let sound_data = StaticSoundData::from_file(filepath)?;

            Ok((key.into(), sound_data))
        })
        .collect::<anyhow::Result<_>>()?;

    let pack = Pack {
        name: pack_name.to_owned(),
        volume: parsed_config.default_volume.parse()?,
        keys: pack_keys,
    };

    Ok(pack)
}
