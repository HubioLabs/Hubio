<script lang="ts">
    export let data;
</script>

{#if data && data.project_info}
    <div class="flex flex-row w-full h-full p-5">
        <div class="card h-full w-1/5 flex flex-col overflow-hidden relative">
            <div
                class="h-1/5"
                style="background-image: url('/project/default_project_cover.webp'); background-size: cover; background-position: center;"
            >
                <div class="flex items-end h-full p-4 bg-gradient-to-t from-surface-900 to-transparent">
                    <h1 class="text-surface-50 text-4xl">{data.project_info.name}</h1>
                </div>
            </div>
        </div>
    </div>
{/if}

