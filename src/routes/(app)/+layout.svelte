<script lang="ts">
	import '@material/web/button/outlined-button';
	import '@material/web/button/filled-button.js';
	import '@material/web/textfield/outlined-text-field.js';
	import type { MdOutlinedTextField } from '@material/web/textfield/outlined-text-field';
	import { signout } from '$lib/bindings';
	import { goto } from '$app/navigation';
	import { storePin } from '$lib/bindings';

	async function signout_user() {
		await signout().catch((err) => {
			console.log(err);
		});
		goto('/signin');
	}

	let pin_element: MdOutlinedTextField;
	let pin_error_message = '';

	function savePin() {
		let new_pin = parseInt(pin_element.value);
		if (isNaN(new_pin)) {
			pin_error_message = 'PIN must be numeric';
			return;
		}
		storePin(new_pin)
			.then(() => {
				data.isPinned = true;
			})
			.catch((err) => {
				console.log(err);
				pin_error_message = 'Some error occured...';
			});
	}

	export let data;
</script>

<div class="header">
	<div class="text">
		<h3>Hello Tobias!</h3>
	</div>
	<div class="logout">
		<md-outlined-button on:click={signout_user}>Logout</md-outlined-button>
	</div>
</div>

{#if data.isPinned}
	<slot />
{:else}
	<div class="missing-pin">
		<h1>Missing PIN</h1>
		<p>
			No PIN was found on this device. The PIN is used to generate passwords and is NOT sent across
			the internet to the cloud. The PIN must be the same across all your devices so a password is
			generate exactly the same on all devices. The PIN must be completely numeric.
		</p>
		<div class="actions">
			<md-outlined-text-field
				bind:this={pin_element}
				label="PIN"
				type="password"
				error={pin_error_message != ''}
				error-text={pin_error_message}
			/>
			<md-filled-button on:click={savePin}>Save PIN</md-filled-button>
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
