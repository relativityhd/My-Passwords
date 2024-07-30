//import { PUBLIC_TAURI_DEV } from '$env/static/public'

import { versionInfo } from '$lib/bindings';

export const prerender = true;
export const ssr = false; // PUBLIC_TAURI_DEV === 'true'

/** @type {import('./$types').LayoutLoad} */
export async function load() {
	const version_info = await versionInfo();
	return { version_info };
}
