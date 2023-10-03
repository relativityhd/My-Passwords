/* import { getUserBuckets, listUserAccounts } from '$lib/bindings';
import { error } from '@sveltejs/kit';

export async function load() {
	const buckets = await getUserBuckets().catch((err) => {
		console.log(err);
		throw error(500, err);
	});
	const accounts = await listUserAccounts().catch((err) => {
		console.log(err);
		throw error(500, err);
	});
	return { buckets, accounts };
}
 */
