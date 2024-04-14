<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { slide } from 'svelte/transition';
	import '@material/web/icon/icon';
	import type { SearchResult } from '$lib/bindings';

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
					<p>in "{result.bucket.name}"</p>
				</div>
			</div>
			<p>{result.identity}</p>
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
	.result-item::after {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: rgba(0, 0, 0, 0);
		transition: background 0.3s ease;
	}

	.result-item:hover::after {
		background: rgba(0, 0, 0, 0.02);
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
