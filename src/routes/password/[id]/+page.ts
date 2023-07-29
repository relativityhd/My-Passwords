import type { RetrievedSecretAccount } from '$lib/bindings.js';
import { retrieveAccount } from '$lib/bindings.js';
import { error } from '@sveltejs/kit';

export function load({ params }): Promise<RetrievedSecretAccount> {
	console.log(params.id);
	const retrievedAccPromise = retrieveAccount(parseInt(params.id))
		.then((retrievedAcc) => {
			return retrievedAcc.RetrievedSecretAccount;
		})
		.catch((err) => {
			console.log(err);
			throw error(500, err);
		});

	return retrievedAccPromise;
}

export const prerender = false;
