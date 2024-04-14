<script lang="ts">
	import '@material/web/icon/icon';
	import RecentCard from '$lib/cards/RecentCard.svelte';

	let recents = [
		{ username: 'tobiast.h.50@gmail.com', institution: 'Google', icon: 'filter_alt' },
		{ username: 'tobiashoelzer@hotmail.com', institution: 'Microsoft', icon: 'filter_clock' },
		{ username: 'tobiashoelzer@hotmail.com', institution: 'Microsoft', icon: 'filter_clock' },
		{ username: 'tobiashoelzer@hotmail.com', institution: 'Microsoft', icon: 'filter_clock' },
		{ username: 'tobiashoelzer@hotmail.com', institution: 'Microsoft', icon: 'filter_clock' },
		{ username: 'tobiashoelzer@hotmail.com', institution: 'Microsoft', icon: 'filter_clock' },
		{
			username: 'tobias.hoelzer@lehre.dhbw-stuttgart.de',
			institution: 'DHBW',
			icon: 'enhanced_encryption'
		}
	];
</script>

<h3 class="headline">Recent</h3>

<div class="grid-container">
	{#each recents as { username, institution, icon }, i}
		<RecentCard {username} {institution} {icon} />
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
