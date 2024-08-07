import { goto } from '$app/navigation';
import { getMode } from '$lib/bindings';
import { logLoadError } from '$lib/errorutils.js';

export async function load({ params }) {
	const mode = await getMode(params.id).catch(
		logLoadError('app/password/+page.ts:getMode', { id: params.id })
	);
	if (mode === 'Secure') {
		goto(`/password/secure/${params.id}`);
	} else if (mode === 'SuperSecure') {
		goto(`/password/supersecure/${params.id}`);
	} else if (mode === 'Sso') {
		goto(`/password/sso/${params.id}`);
	} else if (mode === 'LegacySecure') {
		goto(`/password/legacy/${params.id}`);
	} else {
		throw new Error(`Unknown mode: ${mode} for account ${params.id}`);
	}
}

export const prerender = false;
