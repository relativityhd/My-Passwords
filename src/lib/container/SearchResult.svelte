<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { slide } from 'svelte/transition';
	import '@material/web/icon/icon';
	import '@material/web/iconbutton/icon-button';
	import '@material/web/iconbutton/filled-icon-button';
	import type { SearchResult } from '$lib/bindings';
	import { goto } from '$app/navigation';

	const dispatch = createEventDispatcher();

	const icons = {
		SuperSecure: 'shield_lock',
		Secure: 'enhanced_encryption',
		Sso: 'apartment',
		LegacySecure: 'lock'
	};

	function selectResult(result: SearchResult) {
		dispatch('select', result);
	}

	export let search_results: SearchResult[] = [];
</script>

<div class="results-list" in:slide={{ duration: 200 }} out:slide={{ duration: 200, delay: 10 }}>
	{#each search_results as result (result.id)}
		<div
			class="result-item"
			on:click={() => selectResult(result)}
			on:keypress={() => selectResult(result)}
			role="button"
			aria-label="Select result"
			tabindex="0"
		>
			<div class="left">
				<md-icon>{icons[result.account_type]}</md-icon>
				<div>
					<h3>{result.institution}</h3>
					{#if result.bucket}
						<p>in "{result.bucket.name}"</p>
					{/if}
				</div>
			</div>
			<div class="right">
				<p>{result.identity}</p>
				<md-filled-icon-button
					on:click|stopPropagation={() => goto(`/password/${result.id}`)}
					on:keypress|stopPropagation={() => goto(`/password/${result.id}`)}
					role="button"
					aria-label="Goto password page"
					tabindex="0"
				>
					<md-icon>arrow_forward</md-icon>
				</md-filled-icon-button>
			</div>
		</div>
	{/each}
</div>

<style>
	.results-list {
		background: var(--md-sys-color-primary);
		color: var(--md-sys-color-on-primary);
		border-radius: 12px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		width: 100%;
	}
	.result-item {
		position: relative;
		padding: 13px 16px;
		display: flex;
		justify-content: space-between;
		align-items: center;
		height: 40px;
	}
	.result-item:not(:last-child) {
		border-bottom: 1px solid var(--md-sys-color-on-primary);
	}
	.result-item:hover {
		cursor: pointer;
	}

	.result-item .left {
		flex-grow: 1;
		display: flex;
		align-items: center;
		justify-content: flex-start;
	}

	.result-item .right {
		flex-grow: 1;
		display: flex;
		align-items: center;
		justify-content: flex-end;
	}

	.result-item .right md-filled-icon-button {
		margin-left: 8px;
	}
	.result-item .right md-icon {
		color: var(--md-sys-color-on-primary);
	}

	.result-item h3 {
		margin: 0;
		margin-left: 8px;
	}
	.result-item .left p {
		margin: 0;
		margin-left: 8px;
		font-size: small;
		opacity: 0.8;
	}

	.result-item p {
		margin: 0;
		font-size: small;
		opacity: 0.8;
	}
</style>
