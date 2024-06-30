<script lang="ts">
    import { fade } from "svelte/transition";
    import { exists, BaseDirectory, mkdir, writeTextFile } from '@tauri-apps/plugin-fs';
    import { info, error, warn } from '@tauri-apps/plugin-log';
    import { type ModalSettings, type ModalComponent, getModalStore } from '@skeletonlabs/skeleton';
    import type { ProjectInfo } from '$lib/project/types';
    import NewProjectModal from '$lib/modals/NewProjectModal.svelte';

    export let project_infos: ProjectInfo[] = [];

    export let width: number = 315;
    export let height: number = 425;

    const modalStore = getModalStore();

    const modalComponent: ModalComponent = { ref: NewProjectModal };

    const modal: ModalSettings = {
        type: 'component',
        component: modalComponent,
        response: (project_info: ProjectInfo) => createProject(project_info)
    };

    async function createProject(new_project_info: ProjectInfo) {
        info('Creating a new project');

        const projectsExists = await exists('Hubio/projects', { baseDir: BaseDirectory.Document });
        if (!projectsExists) {
            error('Projects directory does not exist');
            return;
        }

        const projectExists = await exists(`Hubio/projects/${new_project_info.name}`, { baseDir: BaseDirectory.Document });
        if (projectExists) {
            warn('Project with the same name already exists');
            return;
        }

        try {
            await mkdir(`Hubio/projects/${new_project_info.name}`, { baseDir: BaseDirectory.Document });
            await writeTextFile(`Hubio/projects/${new_project_info.name}/info.json`, JSON.stringify(new_project_info), { baseDir: BaseDirectory.Document });
        } catch (e) {
            error(`Failed to create project: ${e}`);
            return;
        }

        // Update the project_infos list
        // Using a reactive assignment to trigger a re-render with the each block
        project_infos = [...project_infos, new_project_info];
    }

    async function newProject() {
        modalStore.trigger(modal);
    }
</script>

<button 
    transition:fade
    on:click={newProject} 
    type="button" 
    class="snap-center card card-hover variant-ghost-tertiary"
    style="width: {width}px; height: {height}px;"
>
    <div class="flex justify-center items-center h-full">
        <i class="fa-regular fa-square-plus fa-2xl"></i>
    </div>
</button>
