/**
 * Represents information about a project.
 */
export interface ProjectInfo {
    name: string;
    description: string;
    created: string;
    modified: string;
    type: ProjectType;
    utilities: ProjectUtilities;
    skeletonOptions: ProjectSkeletonOptions;
}

/**
 * Represents the type of a project.
 */
export enum ProjectType {
    /**
     * SvelteKit configured with TypeScript (recommended).
     */
    TypeScript = "TypeScript",

    /**
     * SvelteKit configured with JavaScript and JSDoc.
     */
    CheckJS = "CheckJS"
}

/**
 * Represents the utilities available for a project.
 */
export interface ProjectUtilities {
    prettier: boolean;    // Add Prettier for code formatting (recommended)
    eslint: boolean;      // Add ESLint for linting (recommended)
    playwright: boolean;  // Add Playwright for testing 
    vitest: boolean;      // Add Vitest for testing
}

/**
 * Represents the available themes for a skeleton.
 * Choose "custom" for a custom theme.
 */
export enum SkeletonTheme {
    Skeleton = "Skeleton",
    Wintry = "Wintry",
    Modern = "Modern",
    Hamlindigo = "Hamlindigo",
    Rocket = "Rocket",
    Sahara = "Sahara",
    GoldNouveau = "GoldNouveau",
    Vintage = "Vintage",
    Seafoam = "Seafoam",
    Crimson = "Crimson",
    Custom = "Custom"
}

/**
 * Represents the options for configuring skeleton in the project.
 */
export interface ProjectSkeletonOptions {
    codeBlocks: boolean;   // Install codeblock optional dependencies
    popups: boolean;       // Install popups optional dependencies
    forms: boolean;        // Install Tailwinds Forms plugin
    typography: boolean;   // Install Tailwinds Typography plugin
    theme: SkeletonTheme;  // Choose the Skeleton theme to use or "custom" for custom theme
}