import { getSecureOverview, getBuckets, inSsoUse } from '$lib/bindings';
import { handleError } from '$lib/errorutils';

export async function load({ params }) {
	const [[account, password], buckets, deletelocked] = await Promise.all([
		getSecureOverview(params.id).catch(
			handleError('app/password/secure/+page.ts:getSecureOverview')
		),
		getBuckets().catch(handleError('app/password/secure/+page.ts:getBuckets')),
		inSsoUse(params.id).catch(handleError('app/password/secure/+page.ts:inSsoUse'))
	]);
	return { id: params.id, account, password, buckets, deletelocked };
}

export const prerender = false;
