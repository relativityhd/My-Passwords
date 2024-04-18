import { getLegacyOverview, inSsoUse } from '$lib/bindings';

export async function load({ params }) {
	const [account, password] = await getLegacyOverview(params.id);
	const deletelocked = await inSsoUse(params.id);
	console.log(account);
	console.log(password);
	return { id: params.id, account, password, deletelocked };
}
