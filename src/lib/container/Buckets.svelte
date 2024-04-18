<script lang="ts">
	import '@material/web/icon/icon';
	import BucketCard from '$lib/cards/BucketCard.svelte';
	import type { Bucket } from '$lib/bindings';

	export let buckets: Bucket[];
</script>

<h3 class="headline">Buckets</h3>

<div class="flex-container">
	{#each buckets as { id, name, color, n }, i}
		<BucketCard {id} {name} {color} {n} />
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
