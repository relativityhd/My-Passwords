// The problem with sveltekit error handling is that any error which is just thrown is handled as an unexpected error (which is fair enough). But this will hide the error and error message on prod, since we don't have the console on prod available... So we need to catch each error, log it and then throw it again with sveltes error function.

import { error as svelteError } from '@sveltejs/kit';
import type { SerializedError } from './types';
import { message } from '@tauri-apps/api/dialog';
import { debug, error } from 'tauri-plugin-log-api';

function gen_text(msg: string, ctx: object | null) {
	let text = `${msg}`;
	if (ctx) {
		text += '\nContext:\n';
		text += JSON.stringify(ctx, null, 2);
	}
	return text;
}

function gen_errtext(place: string, e: SerializedError, ctx: object | null) {
	let text = `[${place} ${e.status}] ${e.message}`;
	if (ctx) {
		text += '\nContext:\n';
		text += JSON.stringify(ctx, null, 2);
	}
	return text;
}

// This function is used to log messages to the console and to a log file - it should not be awaited
export async function logMsg(msg: string, ctx: object | null = null) {
	const text = gen_text(msg, ctx);
	debug(text);
}

// Function for loading data -> Will redirect to the error page and write it to a log file
export function logLoadError(place: string, ctx: object | null = null) {
	return async (e: SerializedError) => {
		const text = gen_errtext(place, e, ctx);
		error(text);
		throw svelteError(e.status, { message: text });
	};
}

// Function for actions -> Will open a popup with the error message and write it to a log file
export function logError(place: string, ctx: object | null = null) {
	return async (e: SerializedError) => {
		let text = gen_errtext(place, e, ctx);
		error(text);

		// This should not fail, because the directory is created in the setup phase
		const { appLogDir } = await import('@tauri-apps/api/path');
		const logDir = await appLogDir();
		text += `\n\nLogs can be found in "${logDir}My-Passwords.log".\nPlease share this error and logs
				with Tobias. Thank you!`;
		message(text, { title: 'MyPasswords - Error', type: 'error' });

		throw svelteError(e.status, { message: e.message });
	};
}
