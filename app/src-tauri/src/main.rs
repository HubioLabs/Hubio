// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use window_vibrancy::apply_acrylic;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_webview_window("main").unwrap();

      apply_acrylic(&window, Some((18, 18, 18, 125)))
          .expect("Unsupported platform! 'apply_acrylic' is only supported on Windows");

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
