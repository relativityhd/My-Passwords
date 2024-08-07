<script lang="ts">
	import '@material/web/button/filled-button';
	import '@material/web/icon/icon';
	import '@material/web/textfield/filled-text-field';
	import type { MdFilledTextField } from '@material/web/textfield/filled-text-field';
	import type { MdCheckbox } from '@material/web/checkbox/checkbox';
	import '@material/web/checkbox/checkbox';
	import { signin } from '$lib/bindings';
	import { goto } from '$app/navigation';
	import type { SerializedError } from '$lib/types';
	import { logLoadError } from '$lib/errorutils';

	let identifierElement: MdFilledTextField;
	let passwordElement: MdFilledTextField;
	let rememberElement: MdCheckbox;

	let isValid = false;

	function handleIdentifierPress(e: KeyboardEvent | Event) {
		passwordElement.setCustomValidity('');
		// Can't use passwordElement.reportValidity() here because it will switch focus to the password field
		identifierElement.reportValidity();
		isValid = identifierElement?.validity.valid && passwordElement?.validity.valid;
		if (e instanceof KeyboardEvent && e.key === 'Enter' && isValid) {
			handleSubmit();
			return;
		}
	}

	function handlePasswordPress(e: KeyboardEvent | Event) {
		passwordElement.setCustomValidity('');
		passwordElement.reportValidity();
		isValid = identifierElement?.validity.valid && passwordElement?.validity.valid;
		if (e instanceof KeyboardEvent && e.key === 'Enter' && isValid) {
			handleSubmit();
			return;
		}
	}

	async function handleSubmit() {
		let identifier = identifierElement.value;
		let password = passwordElement.value;
		let remember = rememberElement.value === 'on';

		const signed = await signin(identifier, password, remember)
			.then(() => {
				return true;
			})
			.catch(async (err: SerializedError) => {
				if (err.status !== 400) {
					return await logLoadError('auth/+page:signin')(err);
				}
				passwordElement.setCustomValidity('Invalid credentials');
				passwordElement.reportValidity();
				isValid = false;
				return false;
			});
		if (signed) {
			goto('/');
		}
	}
</script>

<div class="left">
	<h1>Sign In</h1>
	<p>No account yet? <a href="/signup">Sign Up</a></p>
</div>
<div class="right">
	<div class="inputs">
		<md-filled-text-field
			bind:this={identifierElement}
			label="Email or Name"
			type="text"
			value=""
			required
			on:change={handleIdentifierPress}
			on:keypress={handleIdentifierPress}
			aria-roledescription="identifier"
			role="textbox"
			tabindex="0"
		/>
		<md-filled-text-field
			bind:this={passwordElement}
			label="Password"
			value=""
			type="password"
			required
			on:change={handlePasswordPress}
			on:keypress={handlePasswordPress}
			aria-roledescription="password"
			role="textbox"
			tabindex="0"
		/>
	</div>
	<div class="remember-label">
		<label for="remember"> Remember me </label>
		<md-checkbox id="remember" bind:this={rememberElement} />
	</div>
	<div class="buttons">
		<md-filled-button
			trailing-icon
			disabled={!isValid}
			on:click={handleSubmit}
			on:keypress={handleSubmit}
			role="button"
			tabindex="0">Sign In <md-icon slot="icon">arrow_forward</md-icon></md-filled-button
		>
	</div>
</div>

<style scoped>
	.inputs {
		align-items: flex-start;
		flex-direction: column;
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
	}

	md-filled-text-field {
		width: 300px;
	}

	.remember-label {
		align-items: center;
		display: flex;
		flex-wrap: wrap;
		padding: 16px 0;
		gap: 16px;
		justify-content: flex-end;
	}

	.buttons {
		display: flex;
		flex-flow: row wrap;
		justify-content: flex-end;
		margin-top: 8px;
	}
</style>
