<script lang="ts">
	import '@material/web/icon/icon';
	import BucketCard from '$lib/cards/BucketCard.svelte';
	import type { Bucket } from '$lib/bindings';

	export let buckets: Bucket[];
</script>

<h3 class="headline">Buckets</h3>

<div class="grid-container">
	{#each buckets as { id, name, color, n }, i}
		<BucketCard {id} {name} {color} {n} />
	{/each}
</div>

<style scoped>
	.grid-container {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		grid-gap: 10px;
		grid-auto-flow: dense;
		max-width: 100%;
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
