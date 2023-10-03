<script lang="ts">
	import '@material/web/button/filled-button.js';
	import '@material/web/button/filled-button';
	import '@material/web/textfield/filled-text-field';
	import { signin } from '$lib/bindings';
	import { goto } from '$app/navigation';

	async function handleSubmit(e: SubmitEvent) {
		const formData = new FormData(e.target as HTMLFormElement);
		let email = formData.get('email') as string;
		let password = formData.get('password') as string;
		console.log(formData, email, password);
		await signin(email, password).catch((err) => {
			console.log(err);
		});
		console.log('go to home');
		goto('/');
	}
</script>

<form on:submit|preventDefault={handleSubmit}>
	<div class="row">
		<md-filled-text-field id="email" label="Email" type="email" value="" />
		<md-filled-text-field id="password" label="Password" type="password" value="" />
	</div>
	<div class="row buttons">
		<md-filled-button>Sign In</md-filled-button>
	</div>
	<p>No account yet? <a href="/signup">Signup</a></p>
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
