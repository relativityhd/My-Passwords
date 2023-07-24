import { PUBLIC_TAURI_DEV } from '$env/static/public'
import "carbon-components-svelte/css/all.css";

export const prerender = true
export const ssr = PUBLIC_TAURI_DEV === 'true'
