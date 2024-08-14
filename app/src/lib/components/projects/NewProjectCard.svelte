<script lang="ts">
	import { fade } from 'svelte/transition';
	import { ProjectManager, type Project } from '$lib';
	import { SquarePlus } from 'lucide-svelte';

	export const project_infos: Project[] = [];

	export let width: number = 315;
	export let height: number = 425;

	let showModal: boolean = false;

	let projectManager: ProjectManager = ProjectManager.Cargo;
</script>

<button
	transition:fade
	type="button"
	class="card card-hover snap-center preset-filled-primary-500 preset-outlined-primary-500 preset-opacity-20"
	style="width: {width}px; height: {height}px;"
	on:click={() => (showModal = true)}
>
	<div class="flex h-full items-center justify-center">
		<SquarePlus absoluteStrokeWidth size="35" />
	</div>
</button>

{#if showModal}
	<!-- Modal Overlay -->
	<div
		transition:fade={{ duration: 100 }}
		class="absolute inset-0 flex items-center justify-center backdrop-blur-lg preset-tonal-surface preset-opacity-50"
	>
		<div class="card space-y-4 preset-tonal-surface p-4">
			<!-- Project Name Input -->
			<label class="label">
				<span class="label-text">Name</span>
				<input class="input" type="text" placeholder="Name" />
			</label>
			<!-- Project Manager Selector -->
			<label class="label">
				<span class="label-text">Select</span>
				<select class="select" bind:value={projectManager}>
					<option value={ProjectManager.Cargo}>Cargo</option>
					<option value={ProjectManager.Pnpm}>Pnpm</option>
					<option value={ProjectManager.Yarn}>Yarn</option>
					<option value={ProjectManager.Npm}>Npm</option>
					<option value={ProjectManager.Bun}>Bun</option>
					<option value={ProjectManager.DotNet}>DotNet</option>
				</select>
			</label>
            <!-- Modal Controls -->
            <div class="flex flex-row gap-4">
                <!-- Close Button -->
                <button
					type="button"
                    class="btn preset-filled-surface-500"
                    on:click={() => (showModal = false)}
                >
                    Close
                </button>
                <!-- Create Button -->
                <button
					type="button"
                    class="btn preset-filled-primary-500"
                    on:click={() => (showModal = false)}
                >
                    Create
                </button>
            </div>
		</div>
	</div>
{/if}
