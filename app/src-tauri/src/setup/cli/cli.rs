use super::commands::project::project_init;
use log::{debug, error, warn};
use tauri::App;
use tauri_plugin_cli::CliExt;

pub fn handle_cli(app: &App) -> () {
    match app.cli().matches() {
        // `matches` here is a Struct with { args, subcommand }.
        // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.
        // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
        Ok(matches) => match matches.subcommand {
            Some(subcommand) => {
                debug!("Command: {}", subcommand.name);
                match subcommand.name.as_str() {
                    "init" => {
                        project_init(&app);
                    }
                    _ => {
                        error!("Unknown command: {}", subcommand.name);
                    }
                }
            }
            None => {
                warn!("Start without CLI command");
            }
        },
        Err(_) => {
            error!("Unknown CLI command")
        }
    }
}
