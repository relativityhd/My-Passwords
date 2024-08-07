import { getAllAccounts, getBuckets } from '$lib/bindings';
import { handleError } from '$lib/errorutils.js';

export async function load({ parent }) {
	await parent(); // Wait for connection and authentication
	const [accounts, buckets] = await Promise.all([
		getAllAccounts().catch(handleError('app/list/+page:getAllAccounts')),
		getBuckets().catch(handleError('app/list/+page:getBuckets'))
	]);
	return { accounts, buckets };
}
