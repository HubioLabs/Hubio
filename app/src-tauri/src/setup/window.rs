use log::info;
use tauri::{App, Manager};
use window_vibrancy::apply_acrylic;

pub fn setup_window(app: &App) -> () {
    let window = app.get_webview_window("main").unwrap();

    apply_acrylic(&window, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_acrylic' is only supported on Windows");

    info!("Window setup complete");
}
