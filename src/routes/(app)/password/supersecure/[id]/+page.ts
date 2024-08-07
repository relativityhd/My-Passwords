import { getSupersecureOverview, getBuckets, inSsoUse } from '$lib/bindings';
import { logLoadError } from '$lib/errorutils.js';

export async function load({ params }) {
	const [[account, password], buckets, deletelocked] = await Promise.all([
		getSupersecureOverview(params.id).catch(
			logLoadError('app/password/supersecure/+page.ts:getSupersecureOverview', { id: params.id })
		),
		getBuckets().catch(logLoadError('app/password/supersecure/+page.ts:getBuckets')),
		inSsoUse(params.id).catch(
			logLoadError('app/password/supersecure/+page.ts:inSsoUse', { id: params.id })
		)
	]);

	return { id: params.id, account, password, buckets, deletelocked };
}

export const prerender = false;
