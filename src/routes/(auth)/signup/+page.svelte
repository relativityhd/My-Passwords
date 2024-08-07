<script lang="ts">
	import '@material/web/button/filled-button';
	import '@material/web/icon/icon';
	import '@material/web/textfield/filled-text-field';
	import type { MdFilledTextField } from '@material/web/textfield/filled-text-field';
	import type { MdCheckbox } from '@material/web/checkbox/checkbox';
	import '@material/web/checkbox/checkbox';
	import { signup } from '$lib/bindings';
	import { goto } from '$app/navigation';
	import type { SerializedError } from '$lib/types';
	import { handleError } from '$lib/errorutils';

	let emailElement: MdFilledTextField;
	let usernameElement: MdFilledTextField;
	let passwordElement: MdFilledTextField;
	let rememberElement: MdCheckbox;

	let isValid = false;

	function checkValidityOnElement(e: MdFilledTextField) {
		function checkValidity() {
			passwordElement.setCustomValidity('');
			e.reportValidity();
			isValid =
				emailElement.validity.valid &&
				usernameElement.validity.valid &&
				passwordElement.validity.valid;
		}
		return checkValidity;
	}

	async function handleSubmit() {
		let email = emailElement.value;
		let username = usernameElement.value;
		let password = passwordElement.value;
		let remember = rememberElement.value === 'on';

		await signup(email, username, password, remember).catch((err: SerializedError) => {
			if (err.status === 400) {
				passwordElement.setCustomValidity('Email or Name already exists!');
			} else {
				throw handleError('auth/+page:signup')(err);
			}
			passwordElement.reportValidity();
			isValid = false;
			throw err;
		});
		goto('/');
	}
</script>

<div class="left">
	<h1>Sign Up</h1>
	<p>Already have an account?<br /> <a href="/signin">Sign In</a></p>
</div>

<div class="right">
	<div class="inputs">
		<md-filled-text-field
			bind:this={emailElement}
			label="Email"
			type="email"
			value=""
			required
			on:change={checkValidityOnElement(emailElement)}
			on:input={checkValidityOnElement(emailElement)}
		/>
		<md-filled-text-field
			bind:this={usernameElement}
			label="Name"
			value=""
			required
			on:change={checkValidityOnElement(usernameElement)}
			on:input={checkValidityOnElement(usernameElement)}
		/>
		<md-filled-text-field
			bind:this={passwordElement}
			label="Password"
			type="password"
			value=""
			required
			on:change={checkValidityOnElement(passwordElement)}
			on:input={checkValidityOnElement(passwordElement)}
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
			tabindex="0">Sign Up<md-icon slot="icon">arrow_forward</md-icon></md-filled-button
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
