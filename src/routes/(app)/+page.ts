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

import { getBuckets, listNossoAccounts, getPopular } from '$lib/bindings';
import { handleError } from '$lib/errorutils';

export async function load({ parent }) {
	console.log('load', await parent());
	const [buckets, nosso_accounts_unsorted, populars] = await Promise.all([
		getBuckets().catch(handleError('app/+page.ts:getBuckets')),
		listNossoAccounts().catch(handleError('app/+page.ts:listNossoAccounts')),
		getPopular().catch(handleError('app/+page.ts:populars'))
	]);

	const nosso_accounts = nosso_accounts_unsorted.sort((a, b) =>
		a.institution.localeCompare(b.institution)
	);
	console.log({ buckets, nosso_accounts, populars });
	return { buckets, nosso_accounts, populars };
}
