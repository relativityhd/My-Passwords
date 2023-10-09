<script lang="ts">
	import '@material/web/button/outlined-button';
	import { signout } from '$lib/bindings';
	import { goto } from '$app/navigation';

	async function signout_user() {
		await signout().catch((err) => {
			console.log(err);
		});
		goto('/signin');
	}
</script>

<md-outlined-button on:click={signout_user}>Logout</md-outlined-button>
<slot />
