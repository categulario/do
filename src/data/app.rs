use std::sync::MutexGuard;

use libadwaita as adw;
use libadwaita::prelude::{ApplicationExt, ApplicationExtManual};
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Sender;

use crate::events::handler::Handler;
use crate::events::{DataEvent, EventHandler};
use crate::services::microsoft::service::GraphService;
use crate::services::microsoft::settings::SettingsService;
use crate::services::microsoft::token::TokenService;
use crate::services::ToDoService;

pub struct App {}

impl App {
    pub async fn login() {
        if TokenService::is_token_present() {
            match TokenService::current_token_data() {
                None => println!("Couldn't find current token data"),
                Some(config) => match config.refresh_token().await {
                    Ok(config) => match TokenService::update_token_data(&config) {
                        Ok(_) => println!("Token configuration updated."),
                        Err(err) => println!("{err}"),
                    },
                    Err(err) => println!("{err}"),
                },
            };
        } else {
            match GraphService::authenticate().await {
                Ok(_) => {}
                Err(err) => println!("{err}"),
            }
        }
    }
    pub async fn uri(code: String, data_tx: &MutexGuard<'_, Sender<DataEvent>>) {
        match TokenService::get_token(code).await {
            Ok(token_data) => match TokenService::update_token_data(&token_data) {
                Ok(_) => {
                    match GraphService::get_lists_delta().await {
                        Ok(lists) => {
                            data_tx
                                .send(DataEvent::Login)
                                .await
                                .expect("Failed to send Login event.");
                            data_tx
                                .send(DataEvent::UpdateLists(lists))
                                .await
                                .expect("Failed to send Login event.");
                        }
                        Err(err) => println!("{err}"),
                    }
                    println!("Updated token data.");
                }
                Err(err) => println!("{err}"),
            },
            Err(err) => println!("{err}"),
        }
    }
    pub fn connect_events(application: &adw::Application) -> anyhow::Result<()> {
        let event_handler = EventHandler::new(channel(1), channel(1));
        let ui_handler = event_handler.clone();
        Handler::handle_events(event_handler.clone());
        SettingsService::new()?;
        application
            .connect_open(move |_, files, _| Handler::handle_uri(files, event_handler.clone()));
        application.connect_activate(move |app| Handler::build_ui(app, ui_handler.clone()));
        Ok(())
    }
}
