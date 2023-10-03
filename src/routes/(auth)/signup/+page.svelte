<script lang="ts">
	import '@material/web/button/filled-button.js';
	import '@material/web/button/filled-button';
	import '@material/web/textfield/filled-text-field';
	import { signup } from '$lib/bindings';
	import { goto } from '$app/navigation';

	// Form data
	let email = '';
	let name = '';
	let password = '';

	async function handleSubmit(e: SubmitEvent) {
		const formData = new FormData(e.target as HTMLFormElement);
		let email = formData.get('email') as string;
		let username = formData.get('username') as string;
		let password = formData.get('password') as string;
		console.log(formData, email, password);
		await signup(email, username, password).catch((err) => {
			console.log(err);
		});
		console.log('go to home');
		goto('/');
	}
</script>

<!--
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
 -->

<form on:submit|preventDefault={handleSubmit}>
	<div class="row">
		<md-filled-text-field id="email" label="Email" type="email" value="" />
		<md-filled-text-field id="name" label="Name" value="" />
		<md-filled-text-field id="password" label="Password" type="password" value="" />
	</div>
	<div class="row buttons">
		<md-filled-button>Sign Up</md-filled-button>
	</div>
	<p>Already have an account? <a href="/signin">Signin</a></p>
</form>

<style scoped>
	.row {
		align-items: flex-start;
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
	}
	.buttons {
		justify-content: flex-end;
		padding: 16px;
	}
</style>
