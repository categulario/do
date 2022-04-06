use anyhow::Result;
use async_trait::async_trait;

use crate::models::list::List;
use crate::services::microsoft::models::task::ToDoTask;

pub mod microsoft;

#[async_trait]
pub trait ToDoService<T> {
    // Authentication
    async fn authenticate() -> Result<()>;
    async fn sign_out() -> Result<()>;
    // Lists
    async fn get_lists() -> Result<Vec<List>>;
    async fn get_lists_delta() -> Result<Vec<List>>;
    async fn delete_list(list_id: &str) -> Result<()>;
    async fn post_list(name: String) -> Result<()>;
    async fn update_list(list_id: &str, name: String) -> Result<()>;
    // List groups
    // async fn get_list_groups() -> Result<Vec<List>>;
    // async fn delete_list_groups(list_group_id: &str) -> Result<()>;
    // async fn post_list_groups(list_group_id: &str, group: ListGroup) -> Result<()>;
    // async fn update_list_groups(list_group_id: &str, group: ListGroup) -> Result<()>;
    // Tasks
    async fn get_tasks(list_id: &str) -> Result<Vec<ToDoTask>>;
    async fn get_task(list_id: &str, task_id: &str) -> Result<ToDoTask>;
    async fn delete_task(list_id: &str, task_id: &str) -> Result<()>;
    async fn post_task(list_id: &str, entry: String) -> Result<()>;
    async fn update_task(list_id: &str, task_id: &str, task: ToDoTask) -> Result<()>;
    async fn complete_task(list_id: &str, task_id: &str, completed: bool) -> Result<Vec<ToDoTask>>;
}
