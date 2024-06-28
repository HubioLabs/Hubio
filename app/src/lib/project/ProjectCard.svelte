<script lang="ts">
    import { goto } from '$app/navigation';
    import { fade } from "svelte/transition";
    import type { ProjectInfo } from '$lib/project/i_project_info';

    export let project_info: ProjectInfo;

    export let width: number = 315;
    export let height: number = 425;

    function gotoProjectPage() {
        goto(`/project/${project_info.name}`);
    }
</script>

<button 
    transition:fade
    on:click={gotoProjectPage}
    class="card card-hover overflow-hidden cursor-pointer" 
    style="width: {width}px; height: {height}px; background-image: url('/project/default_project_cover.webp'); background-size: cover; background-position: center;"
>
    <div class="flex items-end h-full p-4 bg-gradient-to-t from-surface-900 to-transparent">
        <h1 class="text-surface-50 text-2xl">{project_info.name}</h1>
    </div>
</button>
