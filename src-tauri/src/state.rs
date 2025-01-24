use anyhow::{anyhow, bail};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::RwLock,
};

use crate::packs::{self, Pack};

pub struct AppState {
    pub muted: RwLock<bool>,
    pub volume: RwLock<Option<u16>>,
    pub current_pack: RwLock<Option<Pack>>,
    pub packs: RwLock<Vec<String>>,
    pub packs_dir: PathBuf,
}

impl AppState {
    pub fn default() -> anyhow::Result<Self> {
        let Some(home) = home::home_dir() else {
            bail!("Cannot find home directory");
        };

        let packs_dir = Path::new(&home).join(crate::APP_NAME);

        match packs_dir.try_exists() {
            Ok(false) => {
                fs::create_dir_all(&packs_dir)
                    .map_err(|_| anyhow!("Couldn't create the packs directory"))?;
            }
            Ok(true) => (),
            Err(e) => bail!(e),
        };

        let packs = packs::list_available(&packs_dir)?;

        Ok(Self {
            muted: RwLock::new(false),
            volume: RwLock::new(None),
            current_pack: RwLock::new(None),
            packs: RwLock::new(packs),
            packs_dir,
        })
    }
}
