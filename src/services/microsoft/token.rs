use cascade::cascade;
use libdmd::config::Config;
use libdmd::format::FileType;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TokenService {
    pub expires_in: usize,
    pub access_token: String,
    pub refresh_token: String,
}

impl TokenService {
    pub(crate) fn is_token_present() -> bool {
        let config = TokenService::current_token_data();
        match config {
            Some(config) => !config.refresh_token.is_empty(),
            None => false,
        }
    }
    pub(crate) fn current_token_data() -> Option<TokenService> {
        Config::get::<TokenService>("do/services/microsoft/token.toml", FileType::TOML)
    }
    pub(crate) fn update_token_data(config: &TokenService) -> anyhow::Result<()> {
        Config::set(
            "do/services/microsoft/token.toml",
            config.clone(),
            FileType::TOML,
        )
    }
    pub(crate) async fn clear_token() -> anyhow::Result<()> {
        let token_data = TokenService::default();
        TokenService::update_token_data(&token_data)
    }
    pub(crate) async fn get_token(code: String) -> anyhow::Result<TokenService> {
        let client = reqwest::Client::new();
        let params = cascade! {
            HashMap::new();
            ..insert("client_id", "af13f4ae-b607-4a07-9ddc-6c5c5d59979f");
            ..insert("scope", "offline_access user.read tasks.read tasks.read.shared tasks.readwrite tasks.readwrite.shared");
            ..insert("redirect_uri", "do://msft/");
            ..insert("grant_type", "authorization_code");
            ..insert("code", code.as_str());
        };
        let response = client
            .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .form(&params)
            .send()
            .await?;
        match response.error_for_status() {
            Ok(response) => {
                let response = response.text().await?;
                let token_data: TokenService = serde_json::from_str(response.as_str())?;
                TokenService::update_token_data(&token_data)?;
                Ok(token_data)
            }
            Err(error) => Err(error.into()),
        }
    }
    pub(crate) async fn refresh_token(mut self) -> anyhow::Result<TokenService> {
        let client = reqwest::Client::new();
        let params = cascade! {
            HashMap::new();
            ..insert("client_id", "af13f4ae-b607-4a07-9ddc-6c5c5d59979f");
            ..insert("scope", "offline_access user.read tasks.read tasks.read.shared tasks.readwrite tasks.readwrite.shared");
            ..insert("redirect_uri", "do://msft/");
            ..insert("grant_type", "refresh_token");
            ..insert("refresh_token", &self.refresh_token);
        };
        let response = client
            .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .form(&params)
            .send()
            .await?;
        match response.error_for_status() {
            Ok(response) => {
                let response = response.text().await?;
                let token_data: TokenService = serde_json::from_str(response.as_str())?;
                TokenService::update_token_data(&token_data)?;
                self = token_data;
                Ok(self)
            }
            Err(error) => Err(error.into()),
        }
    }
}