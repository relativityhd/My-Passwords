<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import '@material/web/icon/icon';
	import type { PopularResult } from '$lib/bindings';

	const icons = {
		SuperSecure: 'shield_lock',
		Secure: 'enhanced_encryption',
		Sso: 'apartment',
		LegacySecure: 'lock'
	};

	const dispatch = createEventDispatcher();

	function selectResult() {
		dispatch('select', popular);
	}

	export let popular: PopularResult;
</script>

<div
	class="grid-item"
	on:click={selectResult}
	on:keypress={selectResult}
	role="button"
	aria-label="Select result"
	tabindex="0"
>
	<div class="content">
		<div class="content-header">
			<h3>{popular.institution}</h3>
			{#if popular.bucket}
				<p>in "{popular.bucket.name}"</p>
			{/if}
		</div>
		{#if popular.account_type === 'Secure' || popular.account_type === 'SuperSecure'}
			<p>{popular.identity}</p>
		{:else if popular.account_type === 'Sso'}
			<p>SSO::{popular.identity}</p>
		{:else}
			<p>Legacy</p>
		{/if}
	</div>
	<div class="icon">
		<md-icon>{icons[popular.account_type]}</md-icon>
	</div>
</div>

<style scoped>
	.grid-item {
		background: var(--md-sys-color-surface-variant);
		color: var(--md-sys-color-on-surface);
		border-radius: 12px;
		padding: 13px 16px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		display: flex;
		justify-content: space-between;
		align-items: center;
		transition: box-shadow 0.1s ease-in-out, transform 0.1s ease-in-out;
		width: 300px;
	}

	.grid-item:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
		transform: scale(1.02);
		cursor: pointer;
	}

	.grid-item .content {
		flex-grow: 1;
	}

	.grid-item h3,
	.grid-item p {
		margin: 0;
	}

	.grid-item p {
		font-size: small;
	}

	.content-header {
		display: flex;
		justify-content: flex-start;
		align-items: end;
	}
	.content-header h3 {
		margin-right: 8px;
	}
	.content-header p {
		margin: 0;
		font-size: small;
		opacity: 0.8;
	}
</style>
