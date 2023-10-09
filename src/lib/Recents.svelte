<script lang="ts">
	import '@material/web/icon/icon';
</script>

<h3>Recently used</h3>

<div class="grid-container">
	<div class="grid-item">
		<h6>tobiast.h.50@gmail.com</h6>
		<p><md-icon>filter_alt</md-icon> Google</p>
	</div>
	<div class="grid-item">
		<h6>tobiashoelzer@hotmail.com</h6>
		<p><md-icon>filter_clock</md-icon> Microsoft</p>
	</div>
	<div class="grid-item">
		<h6>tobias.hoelzer@lehre.dhbw-stuttgart.de</h6>
		<p><md-icon>enhanced_encryption</md-icon> DHBW</p>
	</div>
	<div class="grid-item">
		<h6>tobias.hoelzer@dhbw-engineering.de</h6>
		<p><md-icon>shield_lock</md-icon> DHBW</p>
	</div>
</div>

<style scoped>
	.grid-container {
		overflow-x: scroll;
		display: grid;
		grid-auto-flow: column;
		grid-auto-columns: 150px;
		grid-gap: 16px;
		place-items: stretch;
		place-content: stretch;
		max-width: 100%;
	}

	.grid-item {
		background: var(--md-sys-color-surface-variant);
		color: var(--md-sys-color-on-surface);
		border-radius: 12px;
		padding: 16px;
	}
</style>
