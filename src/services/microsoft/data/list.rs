use std::sync::{Arc, MutexGuard};

use tokio::sync::mpsc::Sender;

use crate::events::DataEvent;
use crate::models::service::MainService;
use crate::services::microsoft::graph::GraphService;
use crate::services::ToDoService;

pub async fn fetch(data_tx: &MutexGuard<'_, Sender<DataEvent>>, service: Arc<MainService>) {
    match service.get_app_lists().await {
        Ok(lists) => data_tx
            .send(DataEvent::UpdateLists(lists))
            .await
            .expect("Failed to send UpdateLists event."),
        Err(err) => println!("{err}"),
    }
}
