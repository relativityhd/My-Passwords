//import { PUBLIC_TAURI_DEV } from '$env/static/public'

import { versionInfo } from '$lib/bindings';
import { attachConsole } from 'tauri-plugin-log-api';

export const prerender = true;
export const ssr = false; // PUBLIC_TAURI_DEV === 'true'

/** @type {import('./$types').LayoutLoad} */
export async function load() {
	attachConsole();
	// Need to dynamicly import sep because of bad design of the window api: https://github.com/tauri-apps/tauri/discussions/5271
	const { appLogDir } = await import('@tauri-apps/api/path');
	const [logDir, version_info] = await Promise.all([appLogDir(), versionInfo()]);
	return { version_info, logDir };
}
