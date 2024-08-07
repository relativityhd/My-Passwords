import { error, redirect } from '@sveltejs/kit';
import { isConnected, isAuthenticated, hasLc, getUsername } from '$lib/bindings';
import { logLoadError, logMsg } from '$lib/errorutils';

/** @type {import('./$types').LayoutLoad} */
export async function load() {
	const connected = await isConnected().catch(logLoadError('app/+layout.ts:isConnected'));
	if (!connected) {
		throw redirect(307, '/seturl');
	}

	const authenticated = await isAuthenticated().catch(
		logLoadError('app/+layout.ts:isAuthenticated')
	);
	if (!authenticated) {
		throw redirect(307, '/signin');
	}

	const [username, isPinned] = await Promise.all([
		getUsername().catch(logLoadError('app/+layout.ts:getUsername')),
		hasLc().catch(logLoadError('app/+layout.ts:hasLc'))
	]);

	return { isPinned, username };
}
