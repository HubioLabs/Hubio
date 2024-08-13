<script lang="ts">
    import { fade } from "svelte/transition";
    import type { Project } from '$lib';
	import { SquarePlus } from "lucide-svelte";

    export const project_infos: Project[] = [];

    export let width: number = 315;
    export let height: number = 425;
</script>

<button 
    transition:fade
    type="button" 
    class="snap-center card card-hover bg-primary-500 preset-outlined-primary-500 bg-opacity-20"
    style="width: {width}px; height: {height}px;"
>
    <div class="flex justify-center items-center h-full">
        <SquarePlus absoluteStrokeWidth size=35 />
    </div>
</button>