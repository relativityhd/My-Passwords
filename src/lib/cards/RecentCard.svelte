<script lang="ts">
	import '@material/web/icon/icon';

	export let username: string;
	export let institution: string;
	export let icon: string;
</script>

<div class="grid-item">
	<div class="content">
		<h3>{institution}</h3>
		<p>{username}</p>
	</div>
	<div class="icon">
		<md-icon>{icon}</md-icon>
	</div>
</div>

<style scoped>
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
</style>
