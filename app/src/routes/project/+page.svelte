<script lang="ts">
    import ProjectCard from '$lib/project/ProjectCard.svelte';
    import NewProjectCard from '$lib/project/NewProjectCard.svelte';

    export let data;
</script>

<div class="card-container snap-mandatory snap-y scroll-smooth overflow-y-auto max-h-full justify-center">
    {#if data.project_infos}
        {#each data.project_infos as project_info}
            <ProjectCard project_info={project_info} />
        {/each}
        <NewProjectCard bind:project_infos={data.project_infos}/>
    {:else}
        <NewProjectCard bind:project_infos={data.project_infos}/>
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
