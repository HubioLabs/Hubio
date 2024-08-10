use tauri::{App, Manager};
use tauri_plugin_log::{Target, TargetKind};
use tauri_plugin_structure_manager::StructureManagerExt;

use window_vibrancy::apply_acrylic;

use log::{info, trace};

fn structure_verification(app: &mut App) -> Result<(), String> {
    info!("Structure Verification ...");

    app.verify_document()?;
    trace!("Document OK");

    info!("Structure Verification DONE");
    Ok(())
}

fn window_setup(app: &mut App) {
    info!("Window Setup ...");

    let window = app.get_webview_window("main").unwrap();

    apply_acrylic(&window, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_acrylic' is only supported on Windows");

    info!("Window Setup DONE");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_structure_manager::init())
        .setup(|app| {
            info!("Hubio Setup ...");

            structure_verification(app)?;

            #[cfg(target_os = "windows")]
            window_setup(app);

            info!("Hubio Setup DONE\nRunning ...");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
