import { getSecureOverview, getBuckets, inSsoUse } from '$lib/bindings';
import { logLoadError } from '$lib/errorutils';

export async function load({ params }) {
	const [[account, password], buckets, deletelocked] = await Promise.all([
		getSecureOverview(params.id).catch(
			logLoadError('app/password/secure/+page.ts:getSecureOverview', { id: params.id })
		),
		getBuckets().catch(logLoadError('app/password/secure/+page.ts:getBuckets')),
		inSsoUse(params.id).catch(
			logLoadError('app/password/secure/+page.ts:inSsoUse', { id: params.id })
		)
	]);
	return { id: params.id, account, password, buckets, deletelocked };
}

export const prerender = false;
