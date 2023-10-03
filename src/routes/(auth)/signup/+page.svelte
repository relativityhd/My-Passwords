<script lang="ts">
	import { Button, Modal } from 'carbon-components-svelte';
	import { FluidForm, TextInput, PasswordInput } from 'carbon-components-svelte';
	import { signup } from '$lib/bindings';
	import { goto } from '$app/navigation';

	// Form data
	let email = '';
	let name = '';
	let password = '';

	async function submit() {
		await signup(email, name, password).catch((err) => {
			console.log(err);
		});
		goto('/');
	}
</script>

<FluidForm>
	<TextInput
		bind:value={email}
		labelText="Email"
		placeholder="Your primary email adress"
		required
	/>
	<TextInput bind:value={name} labelText="Name" placeholder="Your name" required />
	<PasswordInput bind:value={password} labelText="Password" placeholder="Your password" required />
	<Button on:click={submit}>Sign Up</Button>
</FluidForm>

<p>Already got an account? <a href="/signin">Signin</a></p>
