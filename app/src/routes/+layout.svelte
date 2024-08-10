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
<div class="flex flex-col w-full h-full bg-surface-950 bg-opacity-50">
	<!-- Title bar with drag region -->
	<div data-tauri-drag-region class="flex flex-row items-center h-10 pl-4 pr-4">
		<!-- Title -->
		<h1 class="text-xl mr-auto select-none">
			Hubio
		</h1>

		<!-- Window controls -->
		<div class="flex flex-row gap-6">
			<!-- Minimize button -->
			<button 
				class="select-none hover:bg-surface-500 rounded-sm transition duration-200" 
				on:click={async () => await appWindow.minimize()}
			>
				<Minus absoluteStrokeWidth size=20 />
			</button>

			{#if fullscreen}
				<!-- Unfullscreen button -->
				<button 
					class="select-none hover:bg-surface-500 rounded-sm transition duration-200" 
					on:click={async () => {
							fullscreen = !fullscreen; 
							await appWindow.toggleMaximize();
						}
					}
				>
					<Minimize absoluteStrokeWidth size=20 />
				</button>
			{:else}
				<!-- Fullscreen button -->
				<button 
					class="select-none hover:bg-surface-500 rounded-sm transition duration-200" 
					on:click={async () => {
							fullscreen = !fullscreen; 
							await appWindow.toggleMaximize();
						}
					}
				>
					<Maximize absoluteStrokeWidth size=20 />
				</button>
			{/if}

			<!-- Close button -->
			<button 
				class="select-none hover:bg-error-500 rounded-sm transition duration-200" 
				on:click={async () => await appWindow.close()}
			>
				<X absoluteStrokeWidth size=20 />
			</button>
		</div>
	</div>

	<!-- Main content -->
	<div class="w-full h-full p-4 pt-0">
		<main id="main" class="w-full h-full rounded-lg p-2 bg-surface-950 bg-opacity-50">
			<slot />
		</main>
	</div>
</div>
