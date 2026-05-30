mod runtime;
mod pages;
mod state;
mod app;

use libadwaita::prelude::*;
use libadwaita::Application;

const APP_ID: &str = "com.truehub.app";

fn main() {
    runtime::init();
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(|app| {
        app::build_ui(app);
    });

    app.run();
}