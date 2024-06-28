import { exists, BaseDirectory, readTextFile } from '@tauri-apps/plugin-fs';
import { error } from '@tauri-apps/plugin-log';
import type { ProjectInfo } from '$lib/project/i_project_info.js';

export async function load({ params }) {
    const projectsExists = await exists('Hubio/projects', { baseDir: BaseDirectory.Document });
    if (!projectsExists) {
        error('Projects directory does not exist');
        return;
    }
    
    try {
        let project_info_content = await readTextFile(`Hubio/projects/${params.name}/info.json`, { baseDir: BaseDirectory.Document });
        let project_info: ProjectInfo = JSON.parse(project_info_content);
        return { project_info: project_info };
    } catch (e) {
        error(`Failed to load or parse project info for ${params.name}: ${e}`);
        return;
    }
}