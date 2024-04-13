<div class="body">
	<div class="card">
		<div class="card-image-wrapper">
			<img
				class="card-image"
				src="/wallpaper/desert-ribbons.jpg"
				alt="Artisitc Satellite Imagery"
			/>
		</div>
		<div class="card-content">
			<slot />
		</div>
	</div>
</div>

<style scoped>
	.body {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}

	.card {
		background: var(--md-sys-color-secondary-container) no-repeat center center fixed;
		color: var(--md-sys-color-on-surface);
		height: 100%;
		max-width: fit-content;
		border-radius: 12px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
	}

	.card-image-wrapper {
		width: 100%;
		height: 50px;
		overflow: hidden;
		border-radius: 12px;
	}

	.card-image {
		width: 100%;
		height: 100px;
		object-fit: cover;
		border-radius: 12px;
		transform: scale(1.1);
		transition: transform 0.5s;
	}

	/* Add animation to zoom the image slighlty on hover of the card */
	.card:hover .card-image,
	.card:focus .card-image {
		transform: scale(1.15);
	}

	.card-content {
		padding: 16px;
		display: grid;
		grid-template-columns: 1fr 1fr; /* This creates two columns of equal width */
		grid-gap: 10px;
		align-items: center;
		justify-content: center;
	}
</style>
