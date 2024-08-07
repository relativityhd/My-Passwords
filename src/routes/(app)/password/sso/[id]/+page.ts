import { getSsoOverview, getBuckets, inSsoUse, listNossoAccounts } from '$lib/bindings';
import { handleError } from '$lib/errorutils';

export async function load({ params }) {
	const [account, buckets, deletelocked, nosso_accounts_unsorted] = await Promise.all([
		getSsoOverview(params.id).catch(handleError('app/password/sso/+page.ts:getSsoOverview')),
		getBuckets().catch(handleError('app/password/sso/+page.ts:getBuckets')),
		inSsoUse(params.id).catch(handleError('app/password/sso/+page.ts:inSsoUse')),
		listNossoAccounts().catch(handleError('app/password/sso/+page.ts:listNossoAccounts'))
	]);

	const nosso_accounts = nosso_accounts_unsorted.sort((a, b) =>
		a.institution.localeCompare(b.institution)
	);
	return { id: params.id, account, buckets, deletelocked, nosso_accounts };
}

export const prerender = false;
