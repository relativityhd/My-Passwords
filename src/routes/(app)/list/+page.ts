import { getAllAccounts, getBuckets } from '$lib/bindings';
import { logLoadError } from '$lib/errorutils.js';

export async function load({ parent }) {
	await parent(); // Wait for connection and authentication
	const [accounts, buckets] = await Promise.all([
		getAllAccounts().catch(logLoadError('app/list/+page:getAllAccounts')),
		getBuckets().catch(logLoadError('app/list/+page:getBuckets'))
	]);
	return { accounts, buckets };
}
