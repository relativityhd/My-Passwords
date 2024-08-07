<script lang="ts">
	import '@material/web/button/outlined-button';
	import '@material/web/button/filled-button.js';
	import '@material/web/textfield/outlined-text-field.js';
	import type { MdOutlinedTextField } from '@material/web/textfield/outlined-text-field';
	import { signout, storeLc } from '$lib/bindings';
	import { goto } from '$app/navigation';
	import { logLoadError, logMsg } from '$lib/errorutils.js';

	let secret_element: MdOutlinedTextField;
	let pin_element: MdOutlinedTextField;

	async function signout_user() {
		logMsg('Sign out user...');
		await signout().catch(logLoadError('app/+layout.svelte:signout_user'));
		logMsg('Signed out user');
		goto('/signin');
	}

	async function savePin() {
		let new_pin = parseInt(pin_element.value);
		if (isNaN(new_pin)) {
			pin_element.setCustomValidity('PIN must be numeric');
		} else {
			pin_element.setCustomValidity('');
		}
		pin_element.reportValidity();
		secret_element.reportValidity();
		if (!pin_element.validity.valid || !secret_element.validity.valid) {
			return;
		}
		let new_lc = {
			pin: new_pin,
			secret: secret_element.value
		};
		logMsg('Store PIN and Secret...');
		await storeLc(new_lc).catch(logLoadError('app/+layout.svelte:storeLc'));
		logMsg('Stored PIN and Secret');
		data.isPinned = true;
	}

	export let data;
</script>

<div class="header">
	<div class="text">
		<h3>Hello {data.username}!</h3>
	</div>
	<div class="logout">
		<md-outlined-button
			on:click={signout_user}
			on:keypress={signout_user}
			role="button"
			tabindex="0">Logout</md-outlined-button
		>
	</div>
</div>

{#if data.isPinned}
	<slot />
{:else}
	<div class="missing-pin">
		<h1>Missing PIN and Secret</h1>
		<p>
			No PIN and Secret was found on this device. The PIN and Secret are used to generate passwords
			and are NOT sent across the internet to the cloud. The PIN and Secret must be the same across
			all your devices so a password is generate exactly the same on all devices. The PIN must be
			completely numeric and exactly 4 digits long, the secret can be whatever you want.
		</p>
		<div class="actions">
			<md-outlined-text-field
				bind:this={secret_element}
				label="Secret"
				type="password"
				minLength="2"
				maxLength="20"
				required
				on:change={secret_element.reportValidity}
			/>
			<md-outlined-text-field
				bind:this={pin_element}
				label="PIN"
				type="password"
				required
				minLength="4"
				maxLength="4"
				on:change={() => {
					let new_pin = parseInt(pin_element.value);
					if (isNaN(new_pin)) {
						pin_element.setCustomValidity('PIN must be numeric');
					} else {
						pin_element.setCustomValidity('');
					}
					pin_element.reportValidity();
				}}
			/>
			<md-filled-button on:click={savePin} on:keypress={savePin} role="button" tabindex="0"
				>Save PIN</md-filled-button
			>
		</div>
	</div>
{/if}

<style>
	.header {
		display: flex;
		justify-content: flex-end;
		align-items: center;
		padding: 0rem 1rem;
	}

	.text {
		flex: 1;
	}

	.missing-pin {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 1rem;
	}
	.missing-pin .actions {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		gap: 1rem;
	}
</style>
