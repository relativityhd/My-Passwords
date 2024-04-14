import { redirect } from '@sveltejs/kit';
import { isAuthenticated, isPinned } from '$lib/bindings';

/** @type {import('./$types').LayoutLoad} */
export async function load() {
	console.log('check if authenticated');
	if (!(await isAuthenticated())) {
		throw redirect(307, '/signin');
	}
	return { isPinned: isPinned() };
}
