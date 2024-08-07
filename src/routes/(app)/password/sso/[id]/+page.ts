import { getSsoOverview, getBuckets, inSsoUse, listNossoAccounts } from '$lib/bindings';
import { logLoadError } from '$lib/errorutils';

export async function load({ params }) {
	const [account, buckets, deletelocked, nosso_accounts_unsorted] = await Promise.all([
		getSsoOverview(params.id).catch(
			logLoadError('app/password/sso/+page.ts:getSsoOverview', { id: params.id })
		),
		getBuckets().catch(logLoadError('app/password/sso/+page.ts:getBuckets')),
		inSsoUse(params.id).catch(
			logLoadError('app/password/sso/+page.ts:inSsoUse', { id: params.id })
		),
		listNossoAccounts().catch(logLoadError('app/password/sso/+page.ts:listNossoAccounts'))
	]);

	const nosso_accounts = nosso_accounts_unsorted.sort((a, b) =>
		a.institution.localeCompare(b.institution)
	);
	return { id: params.id, account, buckets, deletelocked, nosso_accounts };
}

export const prerender = false;
