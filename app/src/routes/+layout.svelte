<script lang="ts">
	import '../app.css';
	import { Maximize, Minimize, X, Minus } from 'lucide-svelte';
	import { getCurrentWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';

	const appWindow = getCurrentWindow();

	let fullscreen = false;

	onMount(async () => {
		fullscreen = await appWindow.isMaximized();

		// Listen for window resize events
		await listen('tauri://resize', async () => {
			fullscreen = await appWindow.isMaximized();
		});
	});
</script>

<!-- The window -->
<div class="flex h-full w-full flex-col preset-filled-surface-50-950 preset-opacity-50">
	<!-- Title bar with drag region -->
	<div data-tauri-drag-region class="flex h-10 flex-row items-center pl-4 pr-4">
		<!-- Title -->
		<h1 class="mr-auto select-none text-xl">Hubio</h1>

		<!-- Window controls -->
		<div class="flex flex-row gap-6">
			<!-- Minimize button -->
			<button
				type="button"
				class="select-none rounded-sm p-1 transition duration-200 hover:scale-110 hover:preset-tonal-surface hover:preset-opacity-50"
				on:click={async () => await appWindow.minimize()}
			>
				<Minus absoluteStrokeWidth size="20" />
			</button>

			{#if fullscreen}
				<!-- Unfullscreen button -->
				<button
					type="button"
					class="select-none rounded-sm p-1 transition duration-200 hover:scale-110 hover:preset-tonal-surface hover:preset-opacity-50"
					on:click={async () => {
						fullscreen = !fullscreen;
						await appWindow.toggleMaximize();
					}}
				>
					<Minimize absoluteStrokeWidth size="20" />
				</button>
			{:else}
				<!-- Fullscreen button -->
				<button
					type="button"
					class="select-none rounded-sm p-1 transition duration-200 hover:scale-110 hover:preset-tonal-surface hover:preset-opacity-50"
					on:click={async () => {
						fullscreen = !fullscreen;
						await appWindow.toggleMaximize();
					}}
				>
					<Maximize absoluteStrokeWidth size="20" />
				</button>
			{/if}

			<!-- Close button -->
			<button
				type="button"
				class="select-none rounded-sm p-1 transition duration-200 hover:scale-110 hover:preset-filled-error-500"
				on:click={async () => await appWindow.close()}
			>
				<X absoluteStrokeWidth size="20" />
			</button>
		</div>
	</div>

	<!-- Main content -->
	<div class="h-full w-full p-4 pt-0">
		<main id="main" class="relative h-full w-full rounded-lg preset-filled-surface-50-950 preset-opacity-50 p-2 overflow-hidden">
			<slot />
		</main>
	</div>
</div>
