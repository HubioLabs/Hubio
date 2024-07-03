use tauri::Manager;
use tauri_plugin_shell::ShellExt;
use log::{debug, error, info};

use super::project_info::*;

#[tauri::command]
pub async fn new_project(app_handle: tauri::AppHandle, project_info: ProjectInfo) -> Result<(), String> {
    info!("Creating project...");
    debug!("Project info: {:?}", project_info);

    let shell = app_handle.shell();

    let output = 
        shell
            .command("pnpm")
            .args([
                "create",
                "skeleton-app@latest",
                "--quiet",
                "--name", "app", 
                "--path", format!("{}/Hubio/projects/", app_handle.path().document_dir().unwrap().to_str().unwrap()).as_str(),  // TODO: handle possible error instead of unwrap
                "--types", project_info.types.to_string().as_str(),
                // Utilities
                "--eslint", project_info.utilities.eslint.to_string().as_str(),
                "--prettier", project_info.utilities.prettier.to_string().as_str(),
                "--playwright", project_info.utilities.playwright.to_string().as_str(),
                "--vitest", project_info.utilities.vitest.to_string().as_str(),
                // Skeleton options
                "--codeblocks", project_info.skeleton_options.code_blocks.to_string().as_str(),
                "--popups", project_info.skeleton_options.popups.to_string().as_str(),
                "--forms", project_info.skeleton_options.forms.to_string().as_str(),
                "--typography", project_info.skeleton_options.typography.to_string().as_str(),
                "--skeletontemplate", "bare",
                // Theme
                "--skeletontheme", project_info.skeleton_options.theme.to_string().as_str(),
            ])
            .output()
            .await
            .unwrap();

    if output.status.success() {
        info!("Project created successfully");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Failed to create project: {}", stderr);
        Err("Failed to create project".to_string())
    }
}
