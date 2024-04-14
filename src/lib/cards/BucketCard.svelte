<script lang="ts">
	import '@material/web/icon/icon';

	export let name: string;
	export let n: number;
	export let color: string;
	export let id: string;
</script>

<div class="grid-item">
	<div class="card-image-wrapper">
		<img class="card-image" src="/wallpaper/desert-ribbons.jpg" alt="Artisitc Satellite Imagery" />
	</div>
	<div class="card-content">
		<div class="content">
			<h3>{name} ({n})</h3>
		</div>
		<div class="icon">
			<md-icon style="color: {color};">category</md-icon>
		</div>
	</div>
</div>

<style scoped>
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
</style>
