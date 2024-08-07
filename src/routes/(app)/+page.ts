import { getBuckets, listNossoAccounts, getPopular } from '$lib/bindings';
import { logLoadError } from '$lib/errorutils';

export async function load({ parent }) {
	await parent();
	const [buckets, nosso_accounts_unsorted, populars] = await Promise.all([
		getBuckets().catch(logLoadError('app/+page.ts:getBuckets')),
		listNossoAccounts().catch(logLoadError('app/+page.ts:listNossoAccounts')),
		getPopular().catch(logLoadError('app/+page.ts:populars'))
	]);

	const nosso_accounts = nosso_accounts_unsorted.sort((a, b) =>
		a.institution.localeCompare(b.institution)
	);
	return { buckets, nosso_accounts, populars };
}
