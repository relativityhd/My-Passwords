<div class="body">
	<div class="container">
		<slot />
	</div>
</div>

<style scoped>
	.body {
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.container {
		background: var(--md-sys-color-secondary-container);
		color: var(--md-sys-color-on-surface);
		border-radius: 12px;
		padding: 16px;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		max-width: fit-content;
	}
</style>
