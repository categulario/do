use libdmd::config::Config;
use libdmd::{dir, fi};
use libdmd::format::ElementFormat;
use libdmd::element::Element;
use crate::models::token::TokenService;
use crate::services::microsoft::models::token::GraphToken;

pub struct SettingsService {}

impl SettingsService {
    pub fn new() -> anyhow::Result<()> {
        let mut config = Config::new("do")
            .about("Microsoft To Do Client")
            .author("Eduardo Flores")
            .version("0.1.0")
            .write()?;
        if !GraphToken::token_exists() {
            config
                .add(dir!("services")
                    .child(dir!("microsoft")
                        .child(fi!("delta.toml"))
                        .child(fi!("token.toml"))
                    )
                )
                .write()?;
        }
        Ok(())
    }
}