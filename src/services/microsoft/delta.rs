use libdmd::config::Config;
use libdmd::format::FileType;
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Delta {
    #[serde(rename = "@odata.nextLink")]
    next_link: String
}

impl Delta {
    pub fn save(delta_link: String) -> Result<String> {
        let delta = Self {
            next_link: delta_link
        };
        Config::set("do/services/microsoft/delta.toml", delta.clone(), FileType::TOML)?;
        Ok(delta.next_link)
    }
    pub fn current() -> Option<Delta> {
        Config::get::<Delta>("do/services/microsoft/delta.toml", FileType::TOML)
    }
}