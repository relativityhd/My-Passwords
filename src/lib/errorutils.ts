// The problem with sveltekit error handling is that any error which is just thrown is handled as an unexpected error (which is fair enough). But this will hide the error and error message on prod, since we don't have the console on prod available... So we need to catch each error, log it and then throw it again with sveltes error function.

import { error } from '@sveltejs/kit';
import type { SerializedError } from './types';

export function handleError(place: string) {
	return (e: SerializedError) => {
		console.error(new Date().toISOString(), place, e);
		throw error(e.status, e.message);
	};
}
