import { getSecureOverview, getBuckets, inSsoUse } from '$lib/bindings';

export async function load({ params }) {
	const [account, password] = await getSecureOverview(params.id);
	const buckets = await getBuckets();
	const deletelocked = await inSsoUse(params.id);
	console.log(account);
	console.log(password);
	return { id: params.id, account, password, buckets, deletelocked };
}
