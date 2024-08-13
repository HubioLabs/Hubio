<script lang="ts">
	import { AppWindow } from "lucide-svelte";
	import { goto } from '$app/navigation';

	function handleInteraction() {
        goto('/projects');
    }
</script>

<div class="flex justify-center items-center h-full">
	<button
		on:click={handleInteraction} 
		class="hover:bg-surface-500 hover:bg-opacity-50 hover:scale-110 rounded-sm transition duration-200 p-1"
	>
		<AppWindow size="24" />
	</button>
</div>
