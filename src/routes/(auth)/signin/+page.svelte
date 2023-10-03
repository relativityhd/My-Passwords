<script lang="ts">
	import '@material/web/button/filled-button.js';
	import '@material/web/button/filled-button';
	import '@material/web/textfield/filled-text-field';
	import { signin } from '$lib/bindings';
	import { goto } from '$app/navigation';

	let formElement: HTMLFormElement;
	let isValid = false;

	function onChange(e: Event) {
		isValid = formElement.checkValidity();
	}

	async function handleSubmit(e: SubmitEvent) {
		const formData = new FormData(e.target as HTMLFormElement);
		let identifier = formData.get('identifier') as string;
		let password = formData.get('password') as string;
		await signin(identifier, password).catch((err) => {
			console.log(err);
			throw err;
		});
		goto('/');
	}
</script>

<h1>Signin</h1>

<form on:submit|preventDefault={handleSubmit} bind:this={formElement}>
	<div class="inputs">
		<md-filled-text-field
			name="identifier"
			label="Email or Name"
			required
			on:change={onChange}
			on:input={onChange}
		/>
		<md-filled-text-field
			name="password"
			label="Password"
			type="password"
			required
			on:change={onChange}
			on:input={onChange}
		/>
	</div>
	<div class="buttons">
		<md-filled-button disabled={!isValid}>Sign In</md-filled-button>
	</div>
	<p>No account yet? <a href="/signup">Signup</a></p>
</form>

<style scoped>
	.inputs {
		align-items: flex-start;
		flex-direction: column;
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
	}

	.buttons {
		align-items: flex-start;
		display: flex;
		flex-wrap: wrap;
		padding: 16px 0;
		justify-content: flex-end;
	}
</style>
