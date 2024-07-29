import { redirect } from '@sveltejs/kit';
import { isConnected, isAuthenticated, hasLc, getUsername } from '$lib/bindings';

/** @type {import('./$types').LayoutLoad} */
export async function load() {
	console.log('Check if DB is loaded');
	if (!(await isConnected())) {
		throw redirect(307, '/seturl');
	}
	console.log('check if authenticated');
	if (!(await isAuthenticated())) {
		throw redirect(307, '/signin');
	}
	const username = await getUsername();
	return { isPinned: hasLc(), username };
}
