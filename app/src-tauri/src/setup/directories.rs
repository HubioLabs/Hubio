use log::{info, debug};
use tauri::{App, Manager, Wry};
use tauri::path::PathResolver;


pub fn setup_directories(app: &App) -> () {
    let path_resolver = app.path();

    setup_document_dir(path_resolver);

    info!("Directories setup complete");
}

fn setup_document_dir(path_resolver: &PathResolver<Wry>) -> () {
    let document_dir = path_resolver.document_dir().expect("Failed to get document dir");

    debug!("Document dir: {:?}", document_dir);

    let hubio_document_dir = document_dir.join("Hubio");
    if hubio_document_dir.exists() {
        debug!("Hubio document dir already exists");
    } else {
        std::fs::create_dir(&hubio_document_dir).expect("Failed to create Hubio document dir");
        debug!("Hubio document dir created");
    }
}
