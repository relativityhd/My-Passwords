<script lang="ts">
	import { goto } from '$app/navigation';
	import '@material/web/icon/icon';
	import '@material/web/list/list';
	import '@material/web/list/list-item';
	import '@material/web/divider/divider';
	import { DateTime } from 'luxon';

	const icons = {
		SuperSecure: 'shield_lock',
		Secure: 'enhanced_encryption',
		Sso: 'apartment',
		LegacySecure: 'lock'
	};

	import type { ListResult } from '$lib/bindings';
	export let account: ListResult;
</script>

<div
	class="card"
	on:click={() => goto(`/password/${account.id}`)}
	on:keypress={() => goto(`/password/${account.id}`)}
	role="button"
	tabindex="0"
>
	<div class="card-content">
		<div class="card-header">
			<div class="card-header-text">
				<h3>{account.institution}</h3>
				<p>{account.identity}</p>
			</div>
			<md-icon>{icons[account.account_type]}</md-icon>
		</div>
	</div>
</div>

<style>
	.card {
		background: var(--md-sys-color-surface-variant);
		color: var(--md-sys-color-on-surface);
		border-radius: 12px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		transition: box-shadow 0.1s ease-in-out, transform 0.1s ease-in-out;
		width: 300px;
		justify-self: center;
	}

	.card:hover {
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
		transform: scale(1.02);
		cursor: pointer;
	}

	.card-content {
		padding: 13px 16px 16px 16px;
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.card-header-text h3 {
		margin: 0;
		margin-left: 8px;
	}
	.card-header-text p {
		margin: 0;
		margin-left: 8px;
		font-size: small;
		opacity: 0.8;
	}
</style>
