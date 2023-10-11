
use async_trait::async_trait;

#[async_trait]
pub(crate) trait Executing {
    async fn execute_sqlx(
        &mut self,
        query: String,
    ) -> Result<(Vec<String>, Vec<Vec<String>>), sqlx::Error>;
}