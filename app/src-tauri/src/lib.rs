use tauri::Manager;
use window_vibrancy::apply_acrylic;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
