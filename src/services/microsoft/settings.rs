use libdmd::config::Config;
use libdmd::{dir, fi};
use libdmd::format::ElementFormat;
use libdmd::element::Element;
use crate::services::microsoft::token::TokenService;

pub struct SettingsService {}

impl SettingsService {
    pub fn new() -> anyhow::Result<()> {
        let mut config = Config::new("do")
            .about("Microsoft To Do Client")
            .author("Eduardo Flores")
            .version("0.1.0")
            .write()?;
        if !TokenService::is_token_present() {
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