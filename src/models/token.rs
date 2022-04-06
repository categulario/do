pub struct Token {
    pub access_token: String
}
#[async_trait::async_trait]
pub trait TokenService<T: From<Token>> {
    fn token_exists() -> bool;
    async fn create_token(code: Option<String>) -> anyhow::Result<T>;
    fn read_token() -> Option<T>;
    fn update_token(token: &T) -> anyhow::Result<()>;
    fn delete_token() -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl TokenService<Token> for Token {
    fn token_exists() -> bool {
        todo!()
    }

    async fn create_token(code: Option<String>) -> anyhow::Result<Token> {
        todo!()
    }


    fn read_token() -> Option<Token> {
        todo!()
    }

    fn update_token(token: &Token) -> anyhow::Result<()> {
        todo!()
    }

    fn delete_token() -> anyhow::Result<()> {
        todo!()
    }
}