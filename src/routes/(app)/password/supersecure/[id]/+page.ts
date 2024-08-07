import { getSupersecureOverview, getBuckets, inSsoUse } from '$lib/bindings';
import { handleError } from '$lib/errorutils.js';

export async function load({ params }) {
	const [[account, password], buckets, deletelocked] = await Promise.all([
		getSupersecureOverview(params.id).catch(
			handleError('app/password/supersecure/+page.ts:getSupersecureOverview')
		),
		getBuckets().catch(handleError('app/password/supersecure/+page.ts:getBuckets')),
		inSsoUse(params.id).catch(handleError('app/password/supersecure/+page.ts:inSsoUse'))
	]);

	return { id: params.id, account, password, buckets, deletelocked };
}

export const prerender = false;
