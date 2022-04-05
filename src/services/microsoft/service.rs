use anyhow::Context;
use reqwest::StatusCode;
use serde::{Deserialize};

use crate::models::list::List;
use crate::services::microsoft::delta::Delta;
use crate::services::microsoft::task::Task;
use crate::services::microsoft::token::TokenService;
use crate::services::microsoft::types::Collection;
use crate::services::ToDoService;

#[derive(Deserialize)]
pub struct Query {
    pub code: String,
    pub state: String,
}

pub struct GraphService {

}

#[async_trait::async_trait]
impl ToDoService<GraphService> for GraphService {
    async fn authenticate() -> anyhow::Result<()> {
        let url = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?
            client_id=af13f4ae-b607-4a07-9ddc-6c5c5d59979f
            &response_type=code
            &redirect_uri=do://msft/
            &response_mode=query
            &scope=offline_access%20user.read%20tasks.read%20tasks.read.shared%20tasks.readwrite%20tasks.readwrite.shared%20
            &state=1234";
        open::that(url)?;
        Ok(())
    }
    async fn sign_out() -> anyhow::Result<()> {
        TokenService::clear_token().await
    }
    async fn get_lists() -> anyhow::Result<Vec<List>> {
        let config = TokenService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let config = config.refresh_token().await?;
        let client = reqwest::Client::new();
        let response = client
            .get("https://graph.microsoft.com/v1.0/me/todo/lists")
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        match response.error_for_status() {
            Ok(response) => {
                let lists = response.text().await?;
                let lists: Collection<List> = serde_json::from_str(lists.as_str())?;
                Ok(lists.value)
            }
            Err(err) => Err(err.into()),
        }
    }
    async fn get_lists_delta() -> anyhow::Result<Vec<List>> {
        let config = TokenService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let config = config.refresh_token().await?;
        let client = reqwest::Client::new();
        let request = if let Some(delta) = Delta::current() {
            let delta = serde_json::to_string(&delta)?;
            client
                .get("https://graph.microsoft.com/v1.0/me/todo/lists/delta")
                .bearer_auth(&config.access_token)
                .body(delta)
        } else {
            client
                .get("https://graph.microsoft.com/v1.0/me/todo/lists/delta")
                .bearer_auth(&config.access_token)
        };
        let response = request
            .send()
            .await?;
        match response.error_for_status() {
            Ok(response) => {
                let lists = response.text().await?; // TODO: Figure out why it can't parse delta link.
                let lists: Collection<List> = serde_json::from_str(lists.as_str())?;
                Delta::save(lists.delta_link)?;
                Ok(lists.value)
            }
            Err(err) => Err(err.into()),
        }
    }
    async fn delete_list(list_id: &str) -> anyhow::Result<()> {
        let config = TokenService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let config = config.refresh_token().await?;
        let client = reqwest::Client::new();
        let response = client
            .delete(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}",
                list_id
            ))
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        if response.status() == StatusCode::NO_CONTENT {
            return Ok(());
        }
        if let Err(err) = response.error_for_status() {
            return Err(err.into());
        }
        Ok(())
    }
    async fn post_list(name: String) -> anyhow::Result<()> {
        let config = TokenService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let config = config.refresh_token().await?;
        let client = reqwest::Client::new();
        let list = List {
            display_name: name,
            ..std::default::Default::default()
        };
        let data = serde_json::to_string(&list).unwrap();
        let response = client
            .post("https://graph.microsoft.com/v1.0/me/todo/lists")
            .header("Content-Type", "application/json")
            .bearer_auth(&config.access_token)
            .body(data)
            .send()
            .await?;
        if response.status() == StatusCode::CREATED {
            return Ok(());
        }
        if let Err(err) = response.error_for_status() {
            return Err(err.into());
        }
        Ok(())
    }
    async fn update_list(list_id: &str, name: String) -> anyhow::Result<()> {
        let config = TokenService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let config = config.refresh_token().await?;
        let client = reqwest::Client::new();
        let list = List {
            display_name: name,
            ..std::default::Default::default()
        };
        let data = serde_json::to_string(&list).unwrap();
        let response = client
            .patch(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}",
                list_id
            ))
            .header("Content-Type", "application/json")
            .bearer_auth(&config.access_token)
            .body(data)
            .send()
            .await?;
        if response.status() == StatusCode::OK {
            return Ok(());
        }
        if let Err(err) = response.error_for_status() {
            return Err(err.into());
        }
        Ok(())
    }
    async fn get_tasks(task_list_id: &str) -> anyhow::Result<Vec<Task>> {
        let config = TokenService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let config = config.refresh_token().await?;
        let client = reqwest::Client::new();
        let response = client
            .get(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks",
                task_list_id
            ))
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        match response.error_for_status() {
            Ok(response) => {
                let response = response.text().await?;
                let collection: Collection<Task> = serde_json::from_str(response.as_str())?;
                Ok(collection.value)
            }
            Err(error) => Err(error.into()),
        }
    }
    async fn get_task(task_list_id: &str, task_id: &str) -> anyhow::Result<Task> {
        let config = TokenService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let config = config.refresh_token().await?;
        let client = reqwest::Client::new();
        let response = client
            .get(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks/{}",
                task_list_id, task_id
            ))
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        match response.error_for_status() {
            Ok(response) => {
                let response = response.text().await?;
                let task: Task = serde_json::from_str(response.as_str())?;
                Ok(task)
            }
            Err(error) => Err(error.into()),
        }
    }
    async fn delete_task(list_id: &str, task_id: &str) -> anyhow::Result<()> {
        let config = TokenService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let config = config.refresh_token().await?;
        let client = reqwest::Client::new();
        let request = client
            .delete(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks/{}",
                list_id, task_id
            ))
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        if request.status() == StatusCode::NO_CONTENT {
            return Ok(());
        }
        if let Err(err) = request.error_for_status() {
            return Err(err.into());
        }
        Ok(())
    }
    async fn post_task(task_list_id: &str, entry: String) -> anyhow::Result<()> {
        let config = TokenService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let config = config.refresh_token().await?;
        let client = reqwest::Client::new();
        let task = Task {
            title: entry,
            ..std::default::Default::default()
        };
        let data = serde_json::to_string(&task).unwrap();
        let request = client
            .post(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks",
                task_list_id
            ))
            .header("Content-Type", "application/json")
            .bearer_auth(&config.access_token)
            .body(data);
        let response = request.send().await?;
        match response.error_for_status() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
    async fn update_task(list_id: &str, task_id: &str, task: Task) -> anyhow::Result<()> {
        let config = TokenService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let config = config.refresh_token().await?;
        let client = reqwest::Client::new();
        let data = serde_json::to_string(&task).unwrap();
        let response = client
            .patch(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks/{}",
                list_id, task_id
            ))
            .header("Content-Type", "application/json")
            .bearer_auth(&config.access_token)
            .body(data)
            .send()
            .await?;
        if response.status() == StatusCode::OK {
            return Ok(());
        }
        if let Err(err) = response.error_for_status() {
            return Err(err.into());
        }
        Ok(())
    }
    async fn complete_task(
        task_list_id: &str,
        task_id: &str,
        completed: bool,
    ) -> anyhow::Result<Vec<Task>> {
        let config = TokenService::current_token_data()
            .with_context(|| "Failed to get current configuration.")?;
        let config = config.refresh_token().await?;
        let status = format!(
            "{}:{}",
            "{\"status\"",
            if completed {
                "\"notStarted\"}"
            } else {
                "\"completed\"}"
            }
        );
        let client = reqwest::Client::new();
        let response = client
            .patch(format!(
                "https://graph.microsoft.com/v1.0/me/todo/lists/{}/tasks/{}",
                task_list_id, task_id
            ))
            .header("Content-Type", "application/json")
            .body(status)
            .bearer_auth(&config.access_token)
            .send()
            .await?;
        match response.error_for_status() {
            Ok(response) => {
                let response = response.text().await?;
                let collection: Collection<Task> = serde_json::from_str(response.as_str())?;
                Ok(collection.value)
            }
            Err(error) => Err(error.into()),
        }
    }
}
