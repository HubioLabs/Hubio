use std::path::PathBuf;

use log::{error, info, trace, debug};
use tauri::{Manager, Wry};
use tauri_plugin_shell::{Shell, ShellExt};

use crate::models::Project;

#[tauri::command]
/// Creates a project using the specified parameters.
///
/// # Arguments
///
/// * `app_handle` - The handle to the Tauri application.
/// * `project` - The project details.
///
/// # Returns
///
/// Returns `Ok(())` if the project creation is successful, otherwise returns `Err(String)` with an error message.
pub async fn create_project(app_handle: tauri::AppHandle, project: Project) -> Result<(), String> {
    // Log the start of the project creation process
    info!("Project Create: {}", project.name);

    // Log detailed information about the project
    trace!("Project: {:?}", project);

    // Get the shell extension from the app handle
    let shell = app_handle.shell();

    // Get the document directory
    let document_dir = match app_handle.path().document_dir() {
        Ok(dir) => dir,
        Err(_) => return Err("Failed to get document directory".to_string()),
    };

    // Get the path to the project directory
    let project_dir = document_dir.join("Hubio/projects").join(project.name);

    // Make the shell go to the project directory
    goto(shell, project_dir).await?;

    // Execute the command to create a Tauri app with the specified parameters
    let output = shell
        .command("cargo")
        .args([
            "create-tauri-app",
            "--rc",
            "--yes",
            "--m",
            project.manager.to_string().as_str(),
            "--t",
            project.template.to_string().as_str(),
        ])
        .output()
        .await
        .unwrap();

    // Check if the command was successful
    if output.status.success() {
        // Log success message
        info!("Project Create DONE");
        Ok(())
    } else {
        // Log error message with the stderr output
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Project Create FAILED: {}", stderr);
        Err("Project creation failed".to_string())
    }
}

/// Goes to the specified path.
/// 
/// # Arguments
/// 
/// * `app_handle` - The handle to the Tauri application.
/// * `shell` - The shell where the command will be executed.
/// * `path` - The path to go to.
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the goto process is successful, otherwise returns `Err(String)` with an error message.
async fn goto(shell: &Shell<Wry>, path: PathBuf) -> Result<(), String> {
    // Log the start of the goto process
    debug!("Shell go to: {}", path.to_str().unwrap());

    // Check if the path exists
    if !path.exists() {
        // Log error message
        error!("Path does not exist: {}", path.to_str().unwrap());
        return Err("Path does not exist".to_string());
    }

    // Execute the command to go to the specified path
    let output = shell
        .command("cd")
        .arg(path.to_str().unwrap())
        .output()
        .await
        .unwrap();

    // Check if the command was successful
    if output.status.success() {
        // Log success message
        debug!("Shell go to DONE");
        Ok(())
    } else {
        // Log error message with the stderr output
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Shell go to FAILED: {}", stderr);
        Err("Shell go to failed".to_string())
    }
}