<script lang="ts">
	import type { SvelteComponent } from 'svelte';
	import { documentDir } from '@tauri-apps/api/path';
	import { getModalStore } from '@skeletonlabs/skeleton';
    import { ProjectType, type ProjectInfo, type ProjectUtilities, type ProjectSkeletonOptions, SkeletonTheme } from '$lib/project/types';
	import { RadioGroup, RadioItem } from '@skeletonlabs/skeleton';
	import { SlideToggle } from '@skeletonlabs/skeleton';
	import { ListBox, ListBoxItem } from '@skeletonlabs/skeleton';
	import { Accordion, AccordionItem } from '@skeletonlabs/skeleton';

	// Props
	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;

	const modalStore = getModalStore();

	// Base Classes
	const cBase = 'card p-4 w-modal shadow-xl space-y-4';
	const cHeader = 'text-2xl font-bold';
	const cForm = 'border border-surface-500 p-4 space-y-4 rounded-container-token';

    let project_name: string = '';
	let project_type: ProjectType = ProjectType.TypeScript;
	let project_utilities: ProjectUtilities = {
		prettier: true,
		eslint: true,
		playwright: false,
		vitest: false
	};
	let project_skeleton_options: ProjectSkeletonOptions = {
		codeBlocks: false,
		popups: false,
		forms: false,
		typography: false,
		theme: SkeletonTheme.Wintry
	};

    function onFormSubmit(): void {
		let project_info: ProjectInfo = {
			name: project_name,
			description: 'A new amazing Hubio project',
			created: new Date().toISOString(),
			modified: new Date().toISOString(),
			type: project_type,
			utilities: project_utilities,
			skeletonOptions: project_skeleton_options
		};
		if ($modalStore[0].response) $modalStore[0].response(project_info);
		modalStore.close();
	}
</script>

{#if $modalStore[0]}
	<div class="{cBase}">
		<header class={cHeader}>New Project</header>
		<article>The parameters below can be modified at any time.</article>
		<!-- Enable for debugging: -->
		<form class="{cForm}">
			<label class="label">
				<span>Project Name</span>
				<input class="input" type="text" bind:value={project_name} placeholder="Enter project name..." minlength="1" maxlength="32" />
			</label>
			<Accordion autocollapse class="border border-surface-500 rounded-lg">
				<AccordionItem open>
					<svelte:fragment slot="summary">Type</svelte:fragment>
					<svelte:fragment slot="content">
						<RadioGroup>
							<RadioItem bind:group={project_type} name="typescript" value={ProjectType.TypeScript}>TypeScript</RadioItem>
							<RadioItem bind:group={project_type} name="checkjs" value={ProjectType.CheckJS}>JavaScript with JSDoc</RadioItem>
						</RadioGroup>
					</svelte:fragment>
				</AccordionItem>
				<AccordionItem>
					<svelte:fragment slot="summary">Utilities</svelte:fragment>
					<svelte:fragment slot="content">
						<div class="flex gap-5">
							<SlideToggle name="prettier" checked active="bg-surface-500" size="sm">Prettier</SlideToggle>
							<SlideToggle name="eslint" checked active="bg-surface-500" size="sm">Eslint</SlideToggle>
							<SlideToggle name="playwright" active="bg-surface-500" size="sm">Playwright</SlideToggle>
							<SlideToggle name="vitest" active="bg-surface-500" size="sm">Vitest</SlideToggle>
						</div>
					</svelte:fragment>
				</AccordionItem>
				<AccordionItem>
					<svelte:fragment slot="summary">Skeleton Options</svelte:fragment>
					<svelte:fragment slot="content">
						<div class="flex gap-5">
							<SlideToggle name="codeblocks" active="bg-surface-500" size="sm">Code Blocks</SlideToggle>
							<SlideToggle name="popups" active="bg-surface-500" size="sm">Popups</SlideToggle>
							<SlideToggle name="forms" active="bg-surface-500" size="sm">Tailwinds Forms</SlideToggle>
						</div>
						<div class="flex gap-5">
							<SlideToggle name="typography" active="bg-surface-500" size="sm">Tailwinds Typography</SlideToggle>
						</div>
					</svelte:fragment>
				</AccordionItem>
				<AccordionItem>
					<svelte:fragment slot="summary">Theme</svelte:fragment>
					<svelte:fragment slot="content">
						<ListBox>
							<ListBoxItem bind:group={project_skeleton_options.theme} name="skeleton" value={SkeletonTheme.Skeleton}>Skeleton</ListBoxItem>
							<ListBoxItem bind:group={project_skeleton_options.theme} name="wintry" value={SkeletonTheme.Wintry}>Wintry</ListBoxItem>
							<ListBoxItem bind:group={project_skeleton_options.theme} name="modern" value={SkeletonTheme.Modern}>Modern</ListBoxItem>
							<ListBoxItem bind:group={project_skeleton_options.theme} name="hamlindigo" value={SkeletonTheme.Hamlindigo}>Hamlindigo</ListBoxItem>
							<ListBoxItem bind:group={project_skeleton_options.theme} name="rocket" value={SkeletonTheme.Rocket}>Rocket</ListBoxItem>
							<ListBoxItem bind:group={project_skeleton_options.theme} name="sahara" value={SkeletonTheme.Sahara}>Sahara</ListBoxItem>
							<ListBoxItem bind:group={project_skeleton_options.theme} name="gold Nouveau" value={SkeletonTheme.GoldNouveau}>Gold Nouveau</ListBoxItem>
							<ListBoxItem bind:group={project_skeleton_options.theme} name="vintage" value={SkeletonTheme.Vintage}>Vintage</ListBoxItem>
							<ListBoxItem bind:group={project_skeleton_options.theme} name="seafoam" value={SkeletonTheme.Seafoam}>Seafoam</ListBoxItem>
							<ListBoxItem bind:group={project_skeleton_options.theme} name="crimson" value={SkeletonTheme.Crimson}>Crimson</ListBoxItem>
							<ListBoxItem bind:group={project_skeleton_options.theme} name="custom" value={SkeletonTheme.Custom}>Custom</ListBoxItem>
						</ListBox>
					</svelte:fragment>
				</AccordionItem>
			</Accordion>
		</form>
		<!-- prettier-ignore -->
		<footer class="modal-footer {parent.regionFooter}">
			<button class="btn {parent.buttonNeutral}" on:click={parent.onClose}>{parent.buttonTextCancel}</button>
			<button class="btn {parent.buttonPositive}" on:click={onFormSubmit}>Create Project</button>
		</footer>
	</div>
{/if}