use cascade::cascade;
use libdmd::config::Config;
use libdmd::format::FileType;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::models::token::{Token, TokenService};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GraphToken {
    pub expires_in: usize,
    pub access_token: String,
    pub refresh_token: String,
}

impl GraphToken {
    pub(crate) async fn get_token(code: String) -> anyhow::Result<GraphToken> {
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
                let token_data: GraphToken = serde_json::from_str(response.as_str())?;
                GraphToken::update_token(&token_data)?;
                Ok(token_data)
            }
            Err(error) => Err(error.into()),
        }
    }
    pub(crate) async fn refresh_token(mut self) -> anyhow::Result<GraphToken> {
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
                let token_data: GraphToken = serde_json::from_str(response.as_str())?;
                GraphToken::update_token(&token_data)?;
                self = token_data;
                Ok(self)
            }
            Err(error) => Err(error.into()),
        }
    }
}

impl From<Token> for GraphToken {
    fn from(token: Token) -> Self {
        Self {
            expires_in: 0,
            access_token: token.access_token,
            refresh_token: "".to_string()
        }
    }
}

#[async_trait::async_trait]
impl TokenService<GraphToken> for GraphToken {
    fn token_exists() -> bool {
        let config = GraphToken::read_token();
        match config {
            Some(config) => !config.refresh_token.is_empty(),
            None => false,
        }
    }

    async fn create_token(code: Option<String>) -> anyhow::Result<GraphToken> {
        match code {
            None => todo!(),
            Some(code) => GraphToken::get_token(code).await
        }

    }

    fn read_token() -> Option<GraphToken> {
        Config::get::<GraphToken>("do/services/microsoft/token.toml", FileType::TOML)
    }

    fn update_token(token: &GraphToken) -> anyhow::Result<()> {
        Config::set(
            "do/services/microsoft/token.toml",
            token.clone(),
            FileType::TOML,
        )
    }

    fn delete_token() -> anyhow::Result<()> {
        let token_data = GraphToken::default();
        GraphToken::update_token(&token_data)
    }
}