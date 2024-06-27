<script lang="ts">
    import { fade } from "svelte/transition";

    export let project_info;

    export let width: number = 315;
    export let height: number = 425;
</script>

<div 
    transition:fade 
    class="project-card card card-hover overflow-hidden" 
    style="width: {width}px; height: {height}px; background-image: url('/project/default_project_cover.webp'); background-size: cover; background-position: center;"
>
    <div class="flex items-end h-full p-4 bg-gradient-to-t from-surface-900 to-transparent">
        <h1 class="text-surface-50 text-2xl">{project_info.name}</h1>
    </div>
</div>
