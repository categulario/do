use std::sync::Arc;
use adw::prelude::ApplicationExtManual;
use gtk4 as gtk;
use libadwaita as adw;

use crate::data::app::App;
use crate::models::service::MainService;

mod data;
mod events;
mod models;
mod services;
mod ui;
mod traits;

fn main() -> anyhow::Result<()> {
    let application = adw::Application::builder()
        .application_id("do.edfloreshz.github")
        .flags(gtk::gio::ApplicationFlags::HANDLES_OPEN)
        .build();
    let service = Arc::new(MainService::new());
    App::connect_events(&application, service)?;
    application.run();
    Ok(())
}
