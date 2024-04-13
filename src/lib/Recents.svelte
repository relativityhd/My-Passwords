<script lang="ts">
	import '@material/web/icon/icon';

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

<h3 class="headline">Recently used</h3>

<div class="grid-container">
	{#each recents as { username, institution, icon }, i}
		<div class="grid-item">
			<div class="content">
				<h3>{institution}</h3>
				<p>{username}</p>
			</div>
			<div class="icon">
				<md-icon>{icon}</md-icon>
			</div>
		</div>
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
