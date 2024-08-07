/* import type { UnlockedSecretAccount } from '$lib/bindings.js';
import { retrieveSecureAccount } from '$lib/bindings.js';
import { error } from '@sveltejs/kit';

export function load({ params }): Promise<UnlockedSecretAccount> {
	console.log(params.id);
	const retrievedAccPromise = retrieveSecureAccount(parseInt(params.id)).catch((err) => {
		console.log(err);
		throw error(500, err);
	});

	return retrievedAccPromise;
}

export const prerender = false;
 */

import { goto } from '$app/navigation';
import { getMode } from '$lib/bindings';
import { handleError } from '$lib/errorutils.js';

export async function load({ params }) {
	const mode = await getMode(params.id).catch(handleError('app/password/+page.ts:getMode'));
	if (mode === 'Secure') {
		goto(`/password/secure/${params.id}`);
	} else if (mode === 'SuperSecure') {
		goto(`/password/supersecure/${params.id}`);
	} else if (mode === 'Sso') {
		goto(`/password/sso/${params.id}`);
	} else if (mode === 'LegacySecure') {
		goto(`/password/legacy/${params.id}`);
	} else {
		throw new Error(`Unknown mode: ${mode} for account ${params.id}`);
	}
}

export const prerender = false;
