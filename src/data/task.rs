use std::sync::MutexGuard;

use tokio::sync::mpsc::Sender;

use crate::events::DataEvent;
use crate::services::microsoft::service::GraphService;
use crate::services::ToDoService;

pub async fn get_tasks(index: usize, data_tx: &MutexGuard<'_, Sender<DataEvent>>) {
    match GraphService::get_lists_delta().await {
        Ok(lists) => {
            let task_list_id = lists[index].clone().task_list_id;
            match GraphService::get_tasks(task_list_id.as_str()).await {
                Ok(tasks) => data_tx
                    .send(DataEvent::UpdateTasks(task_list_id.clone(), tasks))
                    .await
                    .expect("Failed to send UpdateTasks event."),
                Err(err) => println!("{err}"),
            }
        }
        Err(err) => println!("{err}"),
    }
}

pub async fn set_completed(list_id: String, task_id: String, completed: bool) {
    // TODO: When a task is completed in the details view it needs to be updated in the list view.
    match GraphService::complete_task(list_id.as_str(), task_id.as_str(), completed).await {
        Ok(_) => {}
        Err(err) => println!("{err}"),
    }
}

pub async fn task_selected(
    task_list_id: String,
    task_id: String,
    data_tx: &MutexGuard<'_, Sender<DataEvent>>,
) {
    match GraphService::get_task(&*task_list_id, &*task_id).await {
        Ok(task) => {
            data_tx
                .send(DataEvent::UpdateDetails(task_list_id, Box::from(task)))
                .await
                .expect("Failed to send UpdateLists event.");
        }
        Err(err) => println!("{err}"),
    }
}

pub async fn add_entry(
    entry: String,
    list_id: String,
    data_tx: &MutexGuard<'_, Sender<DataEvent>>,
) {
    match GraphService::post_task(&*list_id.clone(), entry).await {
        Ok(_) => match GraphService::get_tasks(list_id.as_str()).await {
            Ok(tasks) => data_tx
                .send(DataEvent::UpdateTasks(list_id.clone(), tasks))
                .await
                .expect("Failed to send UpdateTasks event."),
            Err(err) => println!("{err}"),
        },
        Err(err) => println!("{err}"),
    }
}

pub async fn add_list_entry(entry: String) {
    match GraphService::post_list(entry).await {
        Ok(_) => {}
        Err(err) => println!("{err}"),
    }
}
