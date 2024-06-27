<script lang="ts">
	import '../app.postcss';

	// Highlight JS
	import hljs from 'highlight.js/lib/core';
	import 'highlight.js/styles/github-dark.css';
	import { storeHighlightJs } from '@skeletonlabs/skeleton';
	import xml from 'highlight.js/lib/languages/xml'; // for HTML
	import css from 'highlight.js/lib/languages/css';
	import javascript from 'highlight.js/lib/languages/javascript';
	import typescript from 'highlight.js/lib/languages/typescript';

	hljs.registerLanguage('xml', xml); // for HTML
	hljs.registerLanguage('css', css);
	hljs.registerLanguage('javascript', javascript);
	hljs.registerLanguage('typescript', typescript);
	storeHighlightJs.set(hljs);

	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

	// Modal
	import { initializeStores, Modal } from '@skeletonlabs/skeleton';
	initializeStores();

	// Log to the console
	import { onMount, onDestroy } from 'svelte';
	import { attachConsole } from '@tauri-apps/plugin-log';
	import type { UnlistenFn } from '@tauri-apps/api/event';

	let dettachConsoleCallback: Promise<UnlistenFn> = new Promise(() => {});

	onMount(() => {
		dettachConsoleCallback = attachConsole();
	});

	onDestroy(() => {
		dettachConsoleCallback.then((dettachConsole) => {
			dettachConsole();
		});
	});
</script>

<Modal />

<div class="p-4 pt-0 w-full h-full">
	<div class="bg-surface-900 w-full h-full rounded-lg bg-opacity-50 overflow-auto overflow-x-hidden">
		<slot />
	</div>
</div>
