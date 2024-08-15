<script lang="ts">
	import { fade } from 'svelte/transition';
	import { ProjectManager, ProjectTemplate, type Project } from '$lib';
	import { SquarePlus } from 'lucide-svelte';

	export const project_infos: Project[] = [];

	export let width: number = 315;
	export let height: number = 425;

	let showModal: boolean = false;

	let projectName: string = '';
	let projectManager: ProjectManager = ProjectManager.Cargo;
	let projectTemplate: ProjectTemplate = ProjectTemplate.Vanilla;

	async function createProject() {
		const project: Project = {
			name: 'New Project',
			description: '',
			thumbnail: null,
			created: new Date(),
			modified: new Date(),
			manager: projectManager,
			template: projectTemplate,
		};

		project_infos.push(project);
	}
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
				<span class="label-text">Project Name</span>
				<input class="input" type="text" placeholder="project name" bind:value={projectName}/>
			</label>
			<!-- Project Template Selector -->
			<label class="label">
				<span class="label-text">Project Template</span>
				<select class="select" bind:value={projectTemplate}>
					<option value={ProjectTemplate.Vanilla}>Vanilla</option>
					<option value={ProjectTemplate.VanillaTS}>Vanilla (TypeScript)</option>
					<option value={ProjectTemplate.Vue}>Vue</option>
					<option value={ProjectTemplate.VueTS}>Vue (TypeScript)</option>
					<option value={ProjectTemplate.Svelte}>Svelte</option>
					<option value={ProjectTemplate.SvelteTS}>Svelte (TypeScript)</option>
					<option value={ProjectTemplate.React}>React</option>
					<option value={ProjectTemplate.ReactTS}>React (TypeScript)</option>
					<option value={ProjectTemplate.Solid}>Solid</option>
					<option value={ProjectTemplate.SolidTS}>Solid (TypeScript)</option>
					<option value={ProjectTemplate.Yew}>Yew</option>
					<option value={ProjectTemplate.Leptos}>Leptos</option>
					<option value={ProjectTemplate.Sycamore}>Sycamore</option>
					<option value={ProjectTemplate.Angular}>Angular</option>
					<option value={ProjectTemplate.Preact}>Preact</option>
					<option value={ProjectTemplate.PreactTS}>Preact (TypeScript)</option>
					<option value={ProjectTemplate.Blazor}>Blazor</option>
				</select>
			<!-- Project Manager Selector -->
			<label class="label">
				<span class="label-text">Project Manager</span>
				<select class="select" bind:value={projectManager}>
					<option value={ProjectManager.Cargo}>Cargo</option>
					<option value={ProjectManager.Pnpm}>Pnpm</option>
					<option value={ProjectManager.Yarn}>Yarn</option>
					<option value={ProjectManager.Npm}>Npm</option>
					<option value={ProjectManager.Bun}>Bun</option>
					<option value={ProjectManager.DotNet}>DotNet</option>
				</select>
			</label>
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
