<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import '@material/web/icon/icon';
	import PopularCard from '$lib/cards/PopularCard.svelte';
	import { getSecurePassword, getSupersecurePassword, getLegacyPassword } from '$lib/bindings';
	import type { PopularResult } from '$lib/bindings';
	import { logError } from '$lib/errorutils';

	const dispatch = createEventDispatcher();

	let getfn = {
		Secure: getSecurePassword,
		SuperSecure: getSupersecurePassword,
		LegacySecure: getLegacyPassword
	};

	async function selectResult(event: CustomEvent<PopularResult>) {
		if (event.detail.account_type === 'Sso') return;
		let fn = getfn[event.detail.account_type];
		if (!fn) return;
		const password = await fn(event.detail.id).catch(
			logError('container/Popular.svelte:selectResult', {
				id: event.detail.id,
				account_type: event.detail.account_type
			})
		);
		dispatch('password', password);
	}

	export let populars: PopularResult[];
</script>

<h3 class="headline">Popular</h3>

<div class="flex-container">
	{#each populars as popular (popular.id)}
		<PopularCard {popular} on:select={selectResult} />
	{/each}
</div>

<style scoped>
	.flex-container {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		gap: 10px;
	}

	.headline {
		font-size: min(15vw, 10rem);
		text-align: center;
		opacity: 0.3;
		margin: -150px 0 0 0;
		position: relative;
		top: 70px;
		z-index: -1;
		white-space: nowrap;
	}
</style>
