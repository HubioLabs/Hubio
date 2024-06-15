use log::{debug, info};
use tauri::path::PathResolver;
use tauri::{App, Manager, Wry};

pub fn setup_directories(app: &App) -> () {
    let path_resolver = app.path();

    setup_document_dir(path_resolver);
    // setup_data_dir(path_resolver);

    info!("Directories setup complete");
}

fn setup_document_dir(path_resolver: &PathResolver<Wry>) -> () {
    let document_dir = path_resolver
        .document_dir()
        .expect("Failed to get document dir");

    let hubio_document_dir = document_dir.join("Hubio");
    if hubio_document_dir.exists() {
        debug!("Hubio document dir already exists");
    } else {
        std::fs::create_dir(&hubio_document_dir).expect("Failed to create Hubio document dir");
        debug!("Hubio document dir created");
    }

    debug!("Document dir: {:?}", hubio_document_dir);
}

fn setup_data_dir(path_resolver: &PathResolver<Wry>) -> () {
    let app_data_dir = path_resolver
        .app_data_dir()
        .expect("Failed to get app data dir");

    debug!("App data dir: {:?}", app_data_dir);

    let apps_dir = app_data_dir.join("apps");
    if apps_dir.exists() {
        debug!("Apps dir already exists");
    } else {
        std::fs::create_dir(&apps_dir).expect("Failed to create apps dir");
        debug!("Apps dir created");
    }

    debug!("Apps dir: {:?}", apps_dir);
}
