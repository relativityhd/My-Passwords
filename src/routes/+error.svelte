<script lang="ts">
	import { page } from '$app/stores';
	import '@material/web/button/outlined-button';
	import '@material/web/button/filled-button';
	import { signout } from '$lib/bindings';
	import { goto } from '$app/navigation';
	import { logError } from '$lib/errorutils';

	async function signout_user() {
		await signout().catch(logError('error.svelte:signout_user'));
		goto('/signin');
	}

	export let data;
</script>

<div class="container">
	<div class="text-container">
		<div class="status-code">{$page.status}</div>
		<div class="error-info-container">
			<h2>Something went wrong</h2>
			<span>Current route: home{$page.url.pathname}</span>
			<p>{$page.error?.message}</p>
			<span>
				Logs can be found in "{data.logDir}My-Passwords.log".<br /> Please share this error and logs
				with Tobias. Thank you!
			</span>
		</div>
	</div>

	<div class="button-row">
		<md-outlined-button
			on:click={signout_user}
			on:keypress={signout_user}
			role="button"
			tabindex="0">Logout</md-outlined-button
		>
		<md-filled-button href="/">Home</md-filled-button>
	</div>
</div>

<style scoped>
	.container {
		max-width: 1000px;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		min-height: calc(100vh - 16px);
	}

	.text-container {
		text-align: left;
		display: flex;
		flex-direction: row;
		align-items: center;
	}

	.status-code {
		font-size: 3rem;
		font-weight: 700;
	}

	.error-info-container {
		opacity: 0.7;
		margin-left: 16px;
		padding-left: 16px;
		border-left: 1px solid var(--md-sys-color-on-surface-variant);
	}

	.error-info-container > h2 {
		margin: 0;
	}

	.error-info-container > span {
		font-size: 0.8rem;
	}

	.error-info-container > p {
		font-size: medium;
	}

	.button-row {
		width: 100%;
		display: flex;
		justify-content: right;
		align-items: right;
		gap: 16px;
		margin-top: 16px;
	}
</style>
