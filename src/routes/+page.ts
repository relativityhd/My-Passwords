import { getUserBuckets } from '$lib/bindings';
import { error } from '@sveltejs/kit';

export async function load() {
	const buckets = await getUserBuckets().catch((err) => {
		console.log(err);
		throw error(500, err);
	});
	const accounts = await getUserAccounts().catch((err) => {
		console.log(err);
		throw error(500, err);
	});
	return { buckets, accounts };
}
