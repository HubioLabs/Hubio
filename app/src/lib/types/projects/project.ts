/**
 * Represents a project.
 */
export interface Project {
    name: string;
    description: string;
    thumbnail: string | null;
    created: Date;
    modified: Date;
    manager: ProjectManager;
    template: ProjectTemplate;
}

/**
 * Represents the available project managers.
 */
export enum ProjectManager {
    Cargo = 'Cargo',
    Pnpm = 'Pnpm',
    Yarn = 'Yarn',
    Npm = 'Npm',
    Bun = 'Bun',
    DotNet = 'DotNet'
}

/**
 * Represents the available project templates.
 */
export enum ProjectTemplate {
    Vanilla = 'Vanilla',
    VanillaTS = 'VanillaTS',
    Vue = 'Vue',
    VueTS = 'VueTS',
    Svelte = 'Svelte',
    SvelteTS = 'SvelteTS',
    React = 'React',
    ReactTS = 'ReactTS',
    Solid = 'Solid',
    SolidTS = 'SolidTS',
    Yew = 'Yew',
    Leptos = 'Leptos',
    Sycamore = 'Sycamore',
    Angular = 'Angular',
    Preact = 'Preact',
    PreactTS = 'PreactTS',
    Blazor = 'Blazor'
}