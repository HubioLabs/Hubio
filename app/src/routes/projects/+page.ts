import { readDir, BaseDirectory, readTextFile } from '@tauri-apps/plugin-fs';
import { warn } from '@tauri-apps/plugin-log';
import type { Project } from '$lib';

/**
 * Loads all projects from the projects directory.
 */
export async function load() {
	const projects: Project[] = [];

	const entries = await readDir('Hubio\\projects', { baseDir: BaseDirectory.Document });
	for (const entry of entries) {
		try {
			const project_info = await readTextFile(`Hubio\\projects\\${entry.name}\\info.json`, {
				baseDir: BaseDirectory.Document
			});
			const project: Project = JSON.parse(project_info);
			projects.push(project);
		} catch (e) {
			warn(`Failed to load or parse project info for ${entry.name}: ${e}`);
		}
	}

	return { projects: projects };
}
