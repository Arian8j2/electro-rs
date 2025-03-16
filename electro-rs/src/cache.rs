use crate::api::ServerName;

use super::api::WgPrivate;
use anyhow::Context;
use std::{
    collections::HashMap,
    env,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

lazy_static::lazy_static! {
    static ref CACHE_PATH: PathBuf = {
        let home_folder =
            PathBuf::from(env::var("HOME").expect("HOME environment variable not exists"));
        let cache_folder = home_folder.join(".cache");
        if !cache_folder.exists() {
            panic!(
                "cache folder doesn't exists at '{}'",
                cache_folder.to_str().unwrap()
            )
        }
        cache_folder.join("electro-rs")
    };
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Cache(HashMap<ServerName, CachedWgPrivate>);

impl Deref for Cache {
    type Target = HashMap<String, CachedWgPrivate>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Cache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CachedWgPrivate {
    pub state: u32,
    pub wg_private: WgPrivate,
}

impl Cache {
    pub fn load() -> anyhow::Result<Self> {
        if CACHE_PATH.exists() {
            let content = std::fs::read_to_string(CACHE_PATH.clone())
                .with_context(|| "couldn't read cache file")?;
            let cache =
                serde_json::from_str(&content).with_context(|| "couldn't deserialize cache")?;
            Ok(cache)
        } else {
            Ok(Cache(HashMap::new()))
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let new_content =
            serde_json::to_string(self).with_context(|| "couldn't serialize cache")?;
        std::fs::write(CACHE_PATH.clone(), new_content)
            .with_context(|| "couldn't write to cache file")
    }
}
