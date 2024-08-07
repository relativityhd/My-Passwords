import { getLegacyOverview, inSsoUse } from '$lib/bindings';
import { handleError } from '$lib/errorutils';

export async function load({ params }) {
	const [[account, password], deletelocked] = await Promise.all([
		getLegacyOverview(params.id).catch(
			handleError('app/password/legacy/+page.ts:getLegacyOverview')
		),
		inSsoUse(params.id).catch(handleError('app/password/legacy/+page.ts:inSsoUse'))
	]);

	return { id: params.id, account, password, deletelocked };
}

export const prerender = false;
