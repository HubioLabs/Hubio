<script lang="ts">
    import { onMount } from 'svelte';
    import { exists, readDir, BaseDirectory, mkdir, readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';
    import { info, error, warn, trace } from '@tauri-apps/plugin-log';
    import ProjectCard from '$lib/project/ProjectCard.svelte';
    import NewProjectCard from '$lib/project/NewProjectCard.svelte';
    import type { ProjectInfo } from '$lib/project/i_project_info';

    let project_infos: ProjectInfo[] = [];

    async function loadProjects() {
        const projectsExists = await exists('Hubio/projects', { baseDir: BaseDirectory.Document });
        if (!projectsExists) {
            error('Projects directory does not exist');
            return;
        }
        
        // Read all project directories in the projects directory
        const entries = await readDir('Hubio/projects', { baseDir: BaseDirectory.Document });
        for (const entry of entries) {
            try {
                let project_info_content = await readTextFile(`Hubio/projects/${entry.name}/info.json`, { baseDir: BaseDirectory.Document });
                let project_info = JSON.parse(project_info_content);
                // Update the project_infos list
                // Using a reactive assignment to trigger a re-render with the each block
                project_infos = [...project_infos, project_info];
            } catch (e) {
                error(`Failed to load or parse project info for ${entry.name}: ${e}`);
            }
        }
    }

    onMount(() => {
        loadProjects();
    });
</script>

<div class="card-container">
    {#each project_infos as project_info}
        <ProjectCard project_info={project_info} />
    {/each}
    <NewProjectCard bind:project_infos/>
</div>

<style>
    .card-container {
        display: grid;
        grid-gap: 20px; /* Space between cards */
        padding: 20px;
        /* Use auto-fill to fill the row with as many cards as it can fit */
        grid-template-columns: repeat(auto-fill, minmax(315px, 1fr));
    }
</style>
