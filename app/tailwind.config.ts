import type { Config } from 'tailwindcss';

import { join } from 'path';
import forms from '@tailwindcss/forms';
import { skeleton } from '@skeletonlabs/skeleton/plugin';
import * as themes from '@skeletonlabs/skeleton/themes';

export default {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		join(require.resolve('@skeletonlabs/skeleton-svelte'), '../**/*.{html,js,svelte,ts}')
	],

	darkMode: 'selector',
	
	theme: {
		extend: {}
	},

	plugins: [
		forms,
		skeleton({
            themes: [ themes.cerberus, themes.rose ]
        })
	]
} as Config;
