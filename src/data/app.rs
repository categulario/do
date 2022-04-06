use std::sync::{Arc, MutexGuard};

use libadwaita as adw;
use libadwaita::prelude::{ApplicationExt, ApplicationExtManual};
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Sender;

use crate::events::handler::Handler;
use crate::events::{DataEvent, EventHandler};
use crate::models::service;
use crate::models::service::MainService;
use crate::models::token::{Token, TokenService};
use crate::services::microsoft::graph::GraphService;
use crate::services::microsoft::models::settings::SettingsService;
use crate::services::microsoft::models::token::GraphToken;
use crate::services::ToDoService;
use crate::traits::app::DoService;

pub struct App {}

impl App {
    pub async fn login(service: Arc<MainService>) {
        if GraphToken::token_exists() {
            match GraphToken::read_token() {
                None => println!("Couldn't find current token data"),
                Some(config) => match config.refresh_token().await {
                    Ok(config) => match GraphToken::update_token(&config) {
                        Ok(_) => println!("Token configuration updated."),
                        Err(err) => println!("{err}"),
                    },
                    Err(err) => println!("{err}"),
                },
            };
        } else {
            match service.authenticate().await {
                Ok(_) => {}
                Err(err) => println!("{err}"),
            }
        }
    }
    pub async fn uri(code: String, data_tx: &MutexGuard<'_, Sender<DataEvent>>, service: Arc<MainService>) {
        match GraphToken::get_token(code).await {
            Ok(token_data) => match GraphToken::update_token(&token_data) {
                Ok(_) => {
                    match service.get_app_lists().await {
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
    pub fn connect_events(application: &adw::Application, service: Arc<MainService>) -> anyhow::Result<()> {
        let event_handler = EventHandler::new(channel(1), channel(1));
        let ui_handler = event_handler.clone();
        Handler::handle_events(event_handler.clone(), service);
        SettingsService::new()?;
        application
            .connect_open(move |_, files, _| Handler::handle_uri(files, event_handler.clone()));
        application.connect_activate(move |app| Handler::build_ui(app, ui_handler.clone()));
        Ok(())
    }
}
