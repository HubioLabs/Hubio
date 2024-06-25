<script lang="ts">
    import { onMount } from 'svelte';
    import { fade } from "svelte/transition";
    import { exists, readDir, BaseDirectory, mkdir, readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';
    import { info, error, warn, trace } from '@tauri-apps/plugin-log';

    interface ProjectInfo {
        name: string;
        description: string;
        created: string;
        modified: string;
    }

    let project_infos: ProjectInfo[] = [];

    async function newProject() {
        info('Creating a new project');

        let new_project_info: ProjectInfo = {
            name: 'New Project',
            description: 'A new project',
            created: new Date().toISOString(),
            modified: new Date().toISOString()
        };

        const projectsExists = await exists('Hubio/projects', { baseDir: BaseDirectory.Document });
        if (!projectsExists) {
            warn('Projects directory does not exist');
            return;
        }

        try {
            await mkdir(`Hubio/projects/${new_project_info.name}`, { baseDir: BaseDirectory.Document });
            await writeTextFile(`Hubio/projects/${new_project_info.name}/info.json`, JSON.stringify(new_project_info), { baseDir: BaseDirectory.Document });
        } catch (e) {
            warn(`Failed to create project: ${e}`);
            return;
        }

        // Update the project_infos list
        // Using a reactive assignment to trigger a re-render with the each block
        project_infos = [...project_infos, new_project_info];
    }

    async function loadProjects() {
        const projectsExists = await exists('Hubio/projects', { baseDir: BaseDirectory.Document });
        if (!projectsExists) {
            warn('Projects directory does not exist');
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
                warn(`Failed to load or parse project info for ${entry.name}: ${e}`);
            }
        }
    }

    onMount(() => {
        loadProjects();
    });
</script>

<style>
    .card-container {
        display: grid;
        grid-gap: 20px; /* Space between cards */
        padding: 20px;
        /* Use auto-fill to fill the row with as many cards as it can fit */
        grid-template-columns: repeat(auto-fill, minmax(315px, 1fr));
    }

    .card-custom {
        width: 315px;
        height: 425px;
        box-sizing: border-box;
    }
</style>

<div class="card-container">
    {#each project_infos as project_info}
        <div class="card-custom block card card-hover" transition:fade>test</div>
    {/each}
    <button type="button" on:click={newProject} class="block card card-custom card-hover variant-ghost-tertiary" transition:fade>
        <div class="flex justify-center items-center h-full">
            <i class="fa-regular fa-square-plus fa-2xl"></i>
        </div>
    </button>
</div>
