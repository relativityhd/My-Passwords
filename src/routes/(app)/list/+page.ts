import { getAllAccounts, getBuckets } from '$lib/bindings';

export async function load() {
	const buckets = await getBuckets();
	const accounts = await getAllAccounts();
	return { accounts, buckets };
}
