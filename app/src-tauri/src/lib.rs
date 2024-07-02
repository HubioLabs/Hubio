use log::info;

use tauri_plugin_log::{Target, TargetKind};

mod setup;
use setup::{cli::cli::handle_cli, directories::setup_directories, window::setup_window};

mod project;
use project::project::new_project;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    info!("Starting Hubio...");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_cli::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir {
                        file_name: Some("Hubio_logs".to_string()),
                    }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            info!("Setting up...");

            setup_window(app);
            setup_directories(app);
            handle_cli(app);

            info!("Setup complete");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![new_project])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    info!("Hubio started");
}
