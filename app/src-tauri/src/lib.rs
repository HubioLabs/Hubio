use log::info;

use tauri_plugin_log::{Target, TargetKind};

mod setup;

use setup::{
    window::setup_window,
    directories::setup_directories,
};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    info!("Starting Hubio...");

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: Some("Hubio_logs".to_string()) }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            info!("Setting up...");

            setup_window(app);
            setup_directories(app);

            info!("Setup complete");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    info!("Hubio started");
}
