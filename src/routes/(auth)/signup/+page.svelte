<script lang="ts">
	import '@material/web/button/filled-button.js';
	import '@material/web/button/filled-button';
	import '@material/web/textfield/filled-text-field';
	import { signup } from '$lib/bindings';
	import { goto } from '$app/navigation';

	let formElement: HTMLFormElement;
	let isValid = false;

	function onChange(e: Event) {
		isValid = formElement.checkValidity();
	}

	async function handleSubmit(e: SubmitEvent) {
		const formData = new FormData(e.target as HTMLFormElement);
		let email = formData.get('email') as string;
		let username = formData.get('username') as string;
		let password = formData.get('password') as string;
		let remember = (formData.get('remember') as string) === 'on';
		await signup(email, username, password, remember).catch((err) => {
			console.log(err);
		});
		console.log('go to home');
		goto('/');
	}
</script>

<div class="left">
	<h1>Sign Up</h1>
	<p>Already have an account?<br /> <a href="/signin">Sign In</a></p>
</div>

<form on:submit|preventDefault={handleSubmit} bind:this={formElement}>
	<div class="inputs">
		<md-filled-text-field
			name="email"
			label="Email"
			type="email"
			required
			on:change={onChange}
			on:input={onChange}
		/>
		<md-filled-text-field
			name="username"
			label="Name"
			value=""
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

	<div class="remember-label">
		<label for="remember"> Remember me </label>
		<md-checkbox id="remember" name="remember" />
	</div>

	<div class="buttons">
		<md-filled-button disabled={!isValid}>Sign Up</md-filled-button>
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
