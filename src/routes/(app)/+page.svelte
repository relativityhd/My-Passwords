<script lang="ts">
	import { fly } from 'svelte/transition';
	import '@material/web/button/filled-button';
	import '@material/web/button/outlined-button';
	import '@material/web/iconbutton/icon-button';
	import '@material/web/textfield/outlined-text-field';
	import '@material/web/icon/icon';
	import '@material/web/list/list';
	import '@material/web/list/list-item';
	import '@material/web/divider/divider';
	import '@material/web/iconbutton/icon-button';
	import '@material/web/tabs/tabs';
	import '@material/web/tabs/secondary-tab';
	import Popular from '$lib/container/Popular.svelte';
	import Buckets from '$lib/container/Buckets.svelte';
	import Search from '$lib/container/Search.svelte';
	import type { MdTabs } from '@material/web/tabs/tabs';
	import CreateSecure from '$lib/forms/CreateSecure.svelte';
	import CreateSuperSecure from '$lib/forms/CreateSuperSecure.svelte';
	import CreateSso from '$lib/forms/CreateSSO.svelte';
	import type { Bucket } from '$lib/bindings.js';

	let password = 'SuperLongandStrongPassword';
	function updatePassword(event: CustomEvent<string>) {
		password = event.detail;
	}

	function copyPassword() {
		navigator.clipboard.writeText(password);
	}

	let activeTabIndex = 0;
	let tabs: MdTabs;

	let activeContent = 'search'; // default content

	export let data;

	let buckets: Bucket[] = [];
	$: if (data) {
		buckets = data.buckets;
	}
</script>

<!-- Password -->
<div class="password">
	<h1>{password}</h1>
	<md-icon-button
		on:click={copyPassword}
		on:keypress={copyPassword}
		role="button"
		aria-label="Copy password"
		tabindex="0"
	>
		<md-icon>content_copy</md-icon>
	</md-icon-button>
</div>

<div class="tab-container">
	{#if activeContent === 'search'}<!-- Note: The grid-items are not in order! -->
		<div
			class="grid-container"
			in:fly={{ x: -200, duration: 500, delay: 500 }}
			out:fly={{ x: -200, duration: 500 }}
		>
			<!-- Search -->
			<div class="search container">
				<Search on:password={updatePassword} />

				<div class="button-row">
					<md-outlined-button href="/list" trailing-icon>
						Manage accounts <md-icon slot="icon">folder_open</md-icon>
					</md-outlined-button>
					<md-filled-button
						trailing-icon
						on:click={() => {
							activeContent = 'new';
						}}
						on:keypress={() => {
							activeContent = 'new';
						}}
						role="button"
						tabindex="0"
					>
						Create new Password <md-icon slot="icon">arrow_forward</md-icon>
					</md-filled-button>
				</div>
			</div>

			<!-- Recents -->
			<div class="last-used container">
				<Popular populars={data.populars} on:password={updatePassword} />
			</div>

			<!-- Buckets -->
			<div class="buckets container">
				<Buckets {buckets} />
			</div>
		</div>
	{:else if activeContent === 'new'}
		<!-- Actions -->
		<div
			class="action container"
			in:fly={{ x: 200, duration: 500, delay: 500 }}
			out:fly={{ x: 200, duration: 500 }}
		>
			<div class="buttons-row">
				<md-outlined-button
					on:click={() => {
						activeContent = 'search';
					}}
					on:keypress={() => {
						activeContent = 'search';
					}}
					role="button"
					tabindex="0"
				>
					<md-icon slot="icon">arrow_back</md-icon> Back to search
				</md-outlined-button>
			</div>
			<md-tabs
				aria-label="Action tabs"
				activeTabIndex={0}
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
				<md-secondary-tab id="tab-three" aria-controls="panel-three">
					<md-icon slot="icon">apartment</md-icon> Sso
				</md-secondary-tab>
			</md-tabs>

			<div role="tabpanel" id="panel-one" aria-labelledby="tab-one" hidden={activeTabIndex !== 0}>
				<CreateSecure on:password={updatePassword} {buckets} />
			</div>
			<div role="tabpanel" id="panel-two" aria-labelledby="tab-two" hidden={activeTabIndex !== 1}>
				<CreateSuperSecure on:password={updatePassword} {buckets} />
			</div>
			<div
				role="tabpanel"
				id="panel-three"
				aria-labelledby="tab-three"
				hidden={activeTabIndex !== 2}
			>
				<CreateSso {buckets} accounts={data.nosso_accounts} />
			</div>
		</div>
	{/if}
</div>

<style scoped>
	.grid-container {
		display: grid;
		grid-template-columns: auto;
		grid-template-rows: auto;
		grid-gap: 16px;
		grid-template-areas:
			'search'
			'last-used'
			'buckets';

		place-items: stretch;
		place-content: stretch;
	}

	.container {
		display: flex;
		flex-flow: column wrap;
		justify-content: center;
		align-items: stretch;
		height: calc(100% - 32px);
		width: calc(100% - 32px);
		padding: 16px;
	}
	.search {
		grid-area: search;
		max-width: 1000px;
		margin: 0 auto;
	}

	.search .button-row {
		display: flex;
		flex-flow: row wrap;
		justify-content: flex-end;
		margin-top: 8px;
		gap: 8px;
	}

	.last-used {
		grid-area: last-used;
	}

	.buckets {
		grid-area: buckets;
	}

	.action {
		max-width: 800px;
		margin: 0 auto;
	}

	.password {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
		height: 20vh; /* Adjust this value as needed */
	}

	.password h1 {
		margin-right: 6px;
	}
</style>
