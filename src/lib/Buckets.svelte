<script lang="ts">
	import '@material/web/icon/icon';

	let buckets = [
		{ name: 'Unsorted', color: 'red', passwords: 120 },
		{ name: 'Work', color: 'blue', passwords: 20 },
		{ name: 'Private', color: 'green', passwords: 10 },
		{ name: 'School', color: 'yellow', passwords: 5 }
	];
</script>

<h3 class="headline">Buckets</h3>

<div class="grid-container">
	{#each buckets as { name, color, passwords }, i}
		<div class="grid-item">
			<div class="card-image-wrapper">
				<img
					class="card-image"
					src="/wallpaper/desert-ribbons.jpg"
					alt="Artisitc Satellite Imagery"
				/>
			</div>
			<div class="card-content">
				<div class="content">
					<h3>{name} ({passwords})</h3>
				</div>
				<div class="icon">
					<md-icon style="color: {color};">token</md-icon>
				</div>
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
		background: var(--md-sys-color-tertiary-container);
		color: var(--md-sys-color-on-surface);
		border-radius: 12px;

		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
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

	.grid-item h3 {
		margin: 0;
	}

	.card-image-wrapper {
		width: 100%;
		height: 150px;
		overflow: hidden;
		border-radius: 12px;
	}
	.card-image {
		width: 100%;
		height: 300px;
		object-fit: cover;
		border-radius: 12px;
		transform: scale(1.1);
		transition: transform 0.5s;
	}
	/* Add animation to zoom the image slighlty on hover of the card */
	.grid-item:hover .card-image,
	.grid-item:focus .card-image {
		transform: scale(1.15);
	}

	.card-content {
		padding: 4px 16px 16px 16px;
		display: flex;
		justify-content: space-between;
		align-items: center;
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
