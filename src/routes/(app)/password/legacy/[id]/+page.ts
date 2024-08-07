import { getLegacyOverview, inSsoUse } from '$lib/bindings';
import { logLoadError } from '$lib/errorutils';

export async function load({ params }) {
	const [[account, password], deletelocked] = await Promise.all([
		getLegacyOverview(params.id).catch(
			logLoadError('app/password/legacy/+page.ts:getLegacyOverview', { id: params.id })
		),
		inSsoUse(params.id).catch(
			logLoadError('app/password/legacy/+page.ts:inSsoUse', { id: params.id })
		)
	]);

	return { id: params.id, account, password, deletelocked };
}

export const prerender = false;
