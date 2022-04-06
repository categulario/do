use anyhow::Result;
use crate::models::list::List;
use crate::models::task::Task;

pub type AppService = Box<dyn DoService>;

#[async_trait::async_trait]
pub trait DoService {
    // Authentication
    async fn authenticate(&self) -> Result<()>;
    async fn sign_out(&self) -> Result<()>;
    // Lists
    async fn get_lists(&self) -> Result<Vec<List>>;
    async fn delete_list(&self, list_id: &str) -> Result<()>;
    async fn post_list(&self, name: String) -> Result<()>;
    async fn update_list(&self, list_id: &str, name: String) -> Result<()>;
    // Tasks
    async fn get_tasks(self: Box<Self>, list_id: &str) -> Result<Vec<Task>>;
    async fn get_task(&self, list_id: &str, task_id: &str) -> Result<Task>;
    async fn delete_task(&self, list_id: &str, task_id: &str) -> Result<()>;
    async fn post_task(&self, list_id: &str, entry: String) -> Result<()>;
    async fn update_task(&self, list_id: &str, task_id: &str, task: Task) -> Result<()>;
}