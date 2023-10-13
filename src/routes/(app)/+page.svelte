<script lang="ts">
	import '@material/web/button/filled-button';
	import '@material/web/button/outlined-button';
	import '@material/web/textfield/outlined-text-field';
	import '@material/web/icon/icon';
	import '@material/web/list/list';
	import '@material/web/list/list-item';
	import '@material/web/divider/divider';
	import '@material/web/iconbutton/icon-button';
	import '@material/web/tabs/tabs';
	import '@material/web/tabs/secondary-tab';
	import Recents from '$lib/Recents.svelte';
	import type { MdTabs } from '@material/web/tabs/tabs';
	import CreateSecure from '$lib/CreateSecure.svelte';

	let activeTabIndex = 0;
	let tabs: MdTabs;
</script>

<h1>Hello Tobi!</h1>

<!-- Note: The grid-items are not in order! -->
<div class="grid-container">
	<!-- Search -->
	<div class="search container">
		<md-outlined-text-field name="search" label="Search">
			<md-icon slot="leading-icon">search</md-icon>
		</md-outlined-text-field>
		<a href="/password/4a8uocok3wqj9jk7y1iv">PW</a>
	</div>

	<!-- Recents -->
	<div class="last-used container">
		<Recents />
	</div>

	<!-- Actions -->
	<div class="action container">
		<md-tabs
			aria-label="Action tabs"
			activeTabIndex={1}
			autoActivate="true"
			on:change={() => (activeTabIndex = tabs.activeTabIndex)}
			bind:this={tabs}
		>
			<md-secondary-tab id="tab-one" aria-controls="panel-one">
				<md-icon slot="icon">enhanced_encryption</md-icon> Secure
			</md-secondary-tab>
			<md-secondary-tab id="tab-two" aria-controls="panel-two">
				<md-icon slot="icon">shield_lock</md-icon> Super Secure
			</md-secondary-tab>
		</md-tabs>

		<div role="tabpanel" id="panel-one" aria-labelledby="tab-one" hidden={activeTabIndex !== 0}>
			<CreateSecure />
		</div>
		<div role="tabpanel" id="panel-two" aria-labelledby="tab-two" hidden={activeTabIndex !== 1}>
			Super Secure
		</div>
	</div>

	<!-- Buckets -->
	<div class="buckets container">
		<h3>Buckets</h3>
		<md-list>
			<md-list-item type="button">
				<div slot="headline">Unsorted</div>
				<div slot="trailing-supporting-text">+100</div>
				<md-icon slot="start">filter_alt</md-icon>
			</md-list-item>
			<md-divider />
			<md-list-item type="button">
				<div slot="headline">Work</div>
				<div slot="trailing-supporting-text">18</div>
				<md-icon slot="start" style="color: blue;">filter_alt</md-icon>
			</md-list-item>
			<md-divider />
			<md-list-item type="button">
				<div slot="headline">DHBW</div>
				<div slot="trailing-supporting-text">3</div>
				<md-icon slot="start" style="color: red;">filter_alt</md-icon>
			</md-list-item>
		</md-list>
	</div>

	<!-- SSO -->
	<div class="sso container">
		<md-outlined-button href="/sso" trailing-icon>
			Manage SSO accounts <md-icon slot="icon">arrow_forward</md-icon>
		</md-outlined-button>
	</div>
</div>

<style scoped>
	.grid-container {
		display: grid;
		grid-template-columns: 3fr 1fr 2fr;
		grid-template-rows: auto;
		grid-gap: 16px;
		grid-template-areas:
			'search search buckets'
			'last-used last-used buckets'
			'last-used last-used sso'
			'action action action';
		place-items: stretch;
		place-content: stretch;
	}

	.search {
		background: var(--md-sys-color-primary-container);
		color: var(--md-sys-color-on-surface);
		border-radius: 12px;
		grid-area: search;
	}
	.action {
		grid-area: action;
	}
	.buckets {
		grid-area: buckets;
	}
	.last-used {
		grid-area: last-used;
	}
	.sso {
		grid-area: sso;
	}

	.container {
		display: flex;
		flex-flow: column wrap;
		justify-content: flex-start;
		align-items: stretch;
		height: calc(100% - 32px);
		width: calc(100% - 32px);
		padding: 16px;
	}
</style>
