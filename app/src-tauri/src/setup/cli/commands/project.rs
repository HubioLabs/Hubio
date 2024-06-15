use tauri::App;
use tauri_plugin_shell::ShellExt;


pub fn project_init(app: &App) -> () {
    let shell = app.shell();

    #[cfg(target_os = "windows")]
    let command = shell.command("cmd");
    #[cfg(target_os = "windows")]
    let args = ["/C", "echo ===== Let's create a new project ====="];

    #[cfg(not(target_os = "windows"))]
    let command = shell.command("echo");
    #[cfg(not(target_os = "windows"))]
    let args = ["===== Let's create a new project ====="];

    let output = tauri::async_runtime::block_on(async move {
        command
            .args(&args)
            .output()
            .await
            .unwrap()
    });

    if output.status.success() {
        println!("Result: {:?}", String::from_utf8(output.stdout));
    } else {
        println!("Exit with code: {}", output.status.code().unwrap());
    }
}
