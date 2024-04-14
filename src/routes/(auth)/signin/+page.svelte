<script lang="ts">
	import '@material/web/button/filled-button';
	import '@material/web/textfield/filled-text-field';
	import '@material/web/checkbox/checkbox';
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
		let remember = (formData.get('remember') as string) === 'on';
		await signin(identifier, password, remember).catch((err) => {
			console.log(err);
			throw err;
		});
		goto('/');
	}
</script>

<div class="left">
	<h1>Sign In</h1>
	<p>No account yet? <a href="/signup">Sign Up</a></p>
</div>

<form on:submit|preventDefault={handleSubmit} bind:this={formElement}>
	<div class="inputs">
		<md-filled-text-field
			name="identifier"
			label="Email or Name"
			type="text"
			value=""
			required
			on:change={onChange}
			on:input={onChange}
		/>
		<md-filled-text-field
			name="password"
			label="Password"
			value=""
			type="password"
			required
			on:change={onChange}
			on:input={onChange}
		/>
	</div>
	<div class="remember-label">
		<label for="remember"> Remember me </label>
		<md-checkbox id="remember" name="remember" />
	</div>
	<div class="buttons">
		<md-filled-button disabled={!isValid}>Sign In</md-filled-button>
	</div>
</form>

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
		align-items: flex-start;
		display: flex;
		flex-wrap: wrap;
		padding: 16px 0;
		justify-content: flex-end;
	}
</style>
