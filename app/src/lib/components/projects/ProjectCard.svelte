<script lang="ts">
    import { goto } from '$app/navigation';
    import { fade } from "svelte/transition";
    import type { Project } from '$lib';

    export let project: Project;

    export let width: number = 315;
    export let height: number = 425;

    let projectThumbnail = project.thumbnail ? `/projects/${project.name}/${project.thumbnail}` : '/projects/default_project_cover.webp';

    function gotoProjectPage() {
        goto(`/projects/${project.name}`);
    }
</script>

<button 
    transition:fade
    on:click={gotoProjectPage}
    class="snap-center card card-hover overflow-hidden cursor-pointer" 
    style="width: {width}px; height: {height}px; background-image: url('{projectThumbnail}'); background-size: cover; background-position: center;"
>
    <div class="flex items-end h-full p-4 bg-gradient-to-t from-surface-950 to-transparent">
        <h1 class="text-surface-50 text-2xl">{project.name}</h1>
    </div>
</button>