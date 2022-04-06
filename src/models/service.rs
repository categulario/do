use anyhow::Result;
use crate::models::list::List;
use crate::models::task::Task;
use async_trait::async_trait;
use crate::services::microsoft::graph::GraphService;
use crate::traits::app::DoService;
use crate::traits::app::AppService;

pub struct MainService {
    pub attached_services: Vec<AppService>
}

impl MainService {
    pub fn new() -> Self {
        Self {
            attached_services: vec![
                Box::new(GraphService {})
            ]
        }
    }
}

unsafe impl Send for MainService {}
unsafe impl Sync for MainService {}

impl MainService {
    pub(crate) async fn get_app_lists(&self) -> Result<Vec<Vec<List>>> {
        let mut lists: Vec<Vec<List>> = vec![];
        for service in &self.attached_services {
            lists.push((*service.get_lists().await?).to_owned());
        }
        Ok(lists)
    }
}

#[async_trait]
impl DoService for MainService {
    async fn authenticate(&self) -> Result<()> {
        todo!()
    }

    async fn sign_out(&self) -> Result<()> {
        todo!()
    }

    async fn get_lists(&self) -> Result<Vec<List>> {
        todo!()
    }

    async fn delete_list(&self, list_id: &str) -> Result<()> {
        todo!()
    }

    async fn post_list(&self, name: String) -> Result<()> {
        todo!()
    }

    async fn update_list(&self, list_id: &str, name: String) -> Result<()> {
        todo!()
    }

    async fn get_tasks(self: Box<Self>, list_id: &str) -> Result<Vec<Task>> {
        todo!()
    }

    async fn get_task(&self, list_id: &str, task_id: &str) -> Result<Task> {
        todo!()
    }

    async fn delete_task(&self, list_id: &str, task_id: &str) -> Result<()> {
        todo!()
    }

    async fn post_task(&self, list_id: &str, entry: String) -> Result<()> {
        todo!()
    }

    async fn update_task(&self, list_id: &str, task_id: &str, task: Task) -> Result<()> {
        todo!()
    }
}