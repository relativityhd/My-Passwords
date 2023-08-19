import type { UnlockedSecretAccount } from '$lib/bindings.js';
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