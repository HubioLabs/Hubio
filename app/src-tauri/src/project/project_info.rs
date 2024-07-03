use std::fmt;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
/// Represents information about a project.
pub struct ProjectInfo {
    pub name: String,
    pub description: String,
    pub created: String,
    pub modified: String,
    pub types: ProjectType,
    pub utilities: ProjectUtilities,
    pub skeleton_options: ProjectSkeletonOptions,
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents the type of a project.
pub enum ProjectType {
    /// SvelteKit configured with TypeScript (recommended).
    TypeScript,

    /// SvelteKit configured with JavaScript and JSDoc.
    CheckJS,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            ProjectType::TypeScript => "typescript",
            ProjectType::CheckJS => "checkjs",
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
/// Represents the available themes for a skeleton.
/// Choose "custom" for a custom theme.
pub enum SkeletonTheme {
    Skeleton,
    Wintry,
    Modern,
    Hamlindigo,
    Rocket,
    Sahara,
    GoldNouveau,
    Vintage,
    Seafoam,
    Crimson,
    Custom
}

impl fmt::Display for SkeletonTheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            SkeletonTheme::Skeleton => "skeleton",
            SkeletonTheme::Wintry => "wintry",
            SkeletonTheme::Modern => "modern",
            SkeletonTheme::Hamlindigo => "hamlindigo",
            SkeletonTheme::Rocket => "rocket",
            SkeletonTheme::Sahara => "sahara",
            SkeletonTheme::GoldNouveau => "gold-nouveau",
            SkeletonTheme::Vintage => "vintage",
            SkeletonTheme::Seafoam => "seafoam",
            SkeletonTheme::Crimson => "crimson",
            SkeletonTheme::Custom => "custom",
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Represents the options for configuring skeleton in the project.
pub struct ProjectSkeletonOptions {
    pub code_blocks: bool,     // Install codeblock optional dependencies
    pub popups: bool,          // Install popups optional dependencies
    pub forms: bool,           // Install Tailwinds Forms plugin
    pub typography: bool,      // Install Tailwinds Typography plugin
    pub theme: SkeletonTheme,  // Choose the Skeleton theme to use or "custom" for custom theme
}