<script lang="ts">
	import { page } from '$app/stores';
	import '@material/web/button/outlined-button';
	import '@material/web/button/filled-button';
	import { signout } from '$lib/bindings';
	import { goto } from '$app/navigation';

	async function signout_user() {
		await signout().catch((err) => {
			console.log(err);
		});
		goto('/signin');
	}
</script>

<h1>{$page.status} Something went wrong :(</h1>
<p>{$page.error?.message}</p>

<md-filled-button href="/">Home</md-filled-button>
<md-outlined-button on:click={signout_user} on:keypress={signout_user} role="button" tabindex="0"
	>Logout</md-outlined-button
>
