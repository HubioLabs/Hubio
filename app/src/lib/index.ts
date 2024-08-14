// place files you want to import through the `$lib` alias in this folder.

export type { Project } from './types/projects/project';
export { ProjectManager, ProjectTemplate } from './types/projects/project';

import ProjectCard from './components/projects/ProjectCard.svelte';
import NewProjectCard from './components/projects/NewProjectCard.svelte';

export { ProjectCard, NewProjectCard };
