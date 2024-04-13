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

<div class="header">
	<div class="text">
		<h3>Hello Tobias!</h3>
	</div>
	<div class="logout">
		<md-outlined-button on:click={signout_user}>Logout</md-outlined-button>
	</div>
</div>
<slot />

<style>
	.header {
		display: flex;
		justify-content: flex-end;
		align-items: center;
		padding: 0rem 1rem;
	}

	.text {
		flex: 1;
	}
</style>
