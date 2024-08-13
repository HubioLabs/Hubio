<script lang="ts">
    import type { Project } from '$lib';
    import { ProjectCard, NewProjectCard } from '$lib';

    export let data: { projects: Project[] };
</script>

<div class="card-container snap-mandatory snap-y scroll-smooth overflow-y-auto max-h-full justify-center">
    {#if data.projects}
        {#each data.projects as project}
            <ProjectCard project={project} />
        {/each}
        <NewProjectCard bind:project_infos={data.projects}/>
    {:else}
        <NewProjectCard bind:project_infos={data.projects}/>
    {/if}
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