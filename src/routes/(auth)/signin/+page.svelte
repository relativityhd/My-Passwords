<script lang="ts">
	import '@material/web/button/filled-button';
	import '@material/web/icon/icon';
	import '@material/web/textfield/filled-text-field';
	import type { MdFilledTextField } from '@material/web/textfield/filled-text-field';
	import type { MdCheckbox } from '@material/web/checkbox/checkbox';
	import '@material/web/checkbox/checkbox';
	import { signin } from '$lib/bindings';
	import { goto } from '$app/navigation';

	let identifierElement: MdFilledTextField;
	let passwordElement: MdFilledTextField;
	let rememberElement: MdCheckbox;

	let isValid = false;

	async function handleSubmit() {
		let identifier = identifierElement.value;
		let password = passwordElement.value;
		let remember = rememberElement.value === 'on';

		console.log(identifierElement?.validity.valid);
		console.log(passwordElement?.validity.valid);

		await signin(identifier, password, remember).catch((err) => {
			console.log(err);
			isValid = false;
			passwordElement.setCustomValidity('Invalid credentials');
			passwordElement.reportValidity();
			throw err;
		});
		goto('/');
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
			on:change={() => {
				passwordElement.setCustomValidity('');
				identifierElement.reportValidity();
				isValid = identifierElement.validity.valid && passwordElement.validity.valid;
			}}
			on:input={() => {
				passwordElement.setCustomValidity('');
				identifierElement.reportValidity();
				isValid = identifierElement.validity.valid && passwordElement.validity.valid;
			}}
		/>
		<md-filled-text-field
			bind:this={passwordElement}
			label="Password"
			value=""
			type="password"
			required
			on:change={() => {
				passwordElement.setCustomValidity('');
				passwordElement.reportValidity();
				isValid = identifierElement.validity.valid && passwordElement.validity.valid;
			}}
			on:input={() => {
				passwordElement.setCustomValidity('');
				passwordElement.reportValidity();
				isValid = identifierElement.validity.valid && passwordElement.validity.valid;
			}}
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
