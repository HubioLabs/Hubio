<script lang="ts">
    import { fade } from "svelte/transition";
    import { exists, BaseDirectory, mkdir, writeTextFile } from '@tauri-apps/plugin-fs';
    import { info, error, warn } from '@tauri-apps/plugin-log';
    import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
    import type { ProjectInfo } from '$lib/project/i_project_info';

    export let project_infos: ProjectInfo[] = [];

    export let width: number = 315;
    export let height: number = 425;

    const modalStore = getModalStore();

    const newProjectModal: ModalSettings = {
        type: 'prompt',
        // Data
        title: 'Project Name',
        body: 'Provide the project name in the field below.',
        // Populates the input value and attributes
        value: '',
        valueAttr: { type: 'text', minlength: 1, maxlength: 16, required: true },
        // Returns the updated response value
        response: async (r: string) => await createProject(r),
    };

    async function createProject(name: string) {
        info('Creating a new project');

        let new_project_info: ProjectInfo = {
            name: name,
            description: 'A new project',
            created: new Date().toISOString(),
            modified: new Date().toISOString()
        };

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
        modalStore.trigger(newProjectModal);
    }
</script>

<button 
    transition:fade
    on:click={newProject} 
    type="button" 
    class="card card-hover variant-ghost-tertiary"
    style="width: {width}px; height: {height}px;"
>
    <div class="flex justify-center items-center h-full">
        <i class="fa-regular fa-square-plus fa-2xl"></i>
    </div>
</button>
