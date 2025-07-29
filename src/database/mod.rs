use async_trait::async_trait;
use crate::errors::core::Result;

pub mod user;
pub mod token;

#[async_trait]
pub trait Repository<T> {
    async fn create(&self, entity: &T) -> Result<T>;
    async fn find_by_id(&self, id: &str) -> Result<Option<T>>;
    async fn update(&self, id: &str, entity: &T) -> Result<T>;
    async fn delete(&self, id: &str) -> Result<bool>;
    async fn find_all(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<T>>;
}
