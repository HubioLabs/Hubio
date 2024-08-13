use std::fmt::Display;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;


#[derive(Serialize, Deserialize, Debug)]
/// Represents a project.
pub struct Project {
    pub name: String,
    pub description: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    pub manager: ProjectManager,
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents the available project managers.
pub enum ProjectManager {
    Cargo,
    Pnpm,
    Yarn,
    Npm,
    Bun,
    DotNet,
}

impl Display for ProjectManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            ProjectManager::Cargo => "cargo",
            ProjectManager::Pnpm => "pnpm",
            ProjectManager::Yarn => "yarn",
            ProjectManager::Npm => "npm",
            ProjectManager::Bun => "bun",
            ProjectManager::DotNet => "dotnet",
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents the utilities available for a project.
pub struct ProjectUtilities {
    pub prettier: bool,    // Add Prettier for code formatting (recommended)
    pub eslint: bool,      // Add ESLint for linting (recommended)
    pub playwright: bool,  // Add Playwright for testing 
    pub vitest: bool,      // Add Vitest for testing
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents the available project templates.
pub enum ProjectTemplate {
    Vanilla,
    VanillaTS,
    Vue,
    VueTS,
    Svelte,
    SvelteTS,
    React,
    ReactTS,
    Solid,
    SolidTS,
    Yew,
    Leptos,
    Sycamore,
    Angular,
    Preact,
    PreactTS,
    Blazor,
}

impl Display for ProjectTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            ProjectTemplate::Vanilla => "vanilla",
            ProjectTemplate::VanillaTS => "vanilla-ts",
            ProjectTemplate::Vue => "vue",
            ProjectTemplate::VueTS => "vue-ts",
            ProjectTemplate::Svelte => "svelte",
            ProjectTemplate::SvelteTS => "svelte-ts",
            ProjectTemplate::React => "react",
            ProjectTemplate::ReactTS => "react-ts",
            ProjectTemplate::Solid => "solid",
            ProjectTemplate::SolidTS => "solid-ts",
            ProjectTemplate::Yew => "yew",
            ProjectTemplate::Leptos => "leptos",
            ProjectTemplate::Sycamore => "sycamore",
            ProjectTemplate::Angular => "angular",
            ProjectTemplate::Preact => "preact",
            ProjectTemplate::PreactTS => "preact-ts",
            ProjectTemplate::Blazor => "blazor",
        })
    }
}
