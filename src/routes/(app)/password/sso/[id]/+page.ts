import { getSsoOverview, getBuckets, inSsoUse, listNossoAccounts } from '$lib/bindings';

export async function load({ params }) {
	const account = await getSsoOverview(params.id);
	const buckets = await getBuckets();
	const deletelocked = await inSsoUse(params.id);
	const nosso_accounts = (await listNossoAccounts()).sort((a, b) =>
		a.institution.localeCompare(b.institution)
	);
	console.log(account);
	return { id: params.id, account, buckets, deletelocked, nosso_accounts };
}

export const prerender = false;
