import { exists, readDir, BaseDirectory, mkdir, readTextFile, writeTextFile } from '@tauri-apps/plugin-fs';
import { error } from '@tauri-apps/plugin-log';
import type { ProjectInfo } from '$lib/project/types';

let project_infos: ProjectInfo[] = [];

export async function load() {
    const projectsExists = await exists('Hubio/projects', { baseDir: BaseDirectory.Document });
    if (!projectsExists) {
        error('Projects directory does not exist');
        return;
    }
    
    // Read all project directories in the projects directory
    const entries = await readDir('Hubio/projects', { baseDir: BaseDirectory.Document });
    for (const entry of entries) {
        try {
            let project_info_content = await readTextFile(`Hubio/projects/${entry.name}/info.json`, { baseDir: BaseDirectory.Document });
            let project_info = JSON.parse(project_info_content);
            project_infos.push(project_info);
        } catch (e) {
            error(`Failed to load or parse project info for ${entry.name}: ${e}`);
        }
    }

    return { project_infos: project_infos };
}