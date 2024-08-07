<script lang="ts">
	import '@material/web/iconbutton/icon-button.js';
	import '@material/web/icon/icon';
	import '@material/web/button/outlined-button.js';
	import '@material/web/divider/divider';
	import AccountCard from '$lib/cards/AccountCard.svelte';
	import CreateBucket from '$lib/forms/CreateBucket.svelte';
	import { deleteBucket, loadFromJson } from '$lib/bindings.js';
	import type { LegacyData, LegacySuperData } from '$lib/bindings.js';
	import { ask, open } from '@tauri-apps/api/dialog';
	import { readTextFile } from '@tauri-apps/api/fs';
	import { logLoadError, logMsg } from '$lib/errorutils.js';

	type ParsedLegacyAccount = {
		acc: string;
		comp: string;
		key: string;
		max: string;
		min: string;
		mode: boolean;
		number: number;
		pin: string;
		spec: string;
		type: string;
		user: string;
		year: string;
		__v: number;
		_id: string;
	};

	async function triggerDelete(id: string) {
		let answer = await ask('Are you sure you want to delete this bucket?', {
			title: 'Delete Bucket',
			type: 'warning'
		});
		if (!answer) return;
		logMsg(`Delete bucket with id ${id}...`);
		await deleteBucket(id).catch(logLoadError('app/list/+page.svelte:triggerDelete'));
		logMsg(`Deleted bucket with id ${id}...`);
		location.reload();
	}

	async function load_json() {
		const f = await open({
			filters: [
				{
					name: 'JSON',
					extensions: ['json']
				}
			]
		});
		if (!f || typeof f !== 'string') return;
		let data = await readTextFile(f);
		let parsed: ParsedLegacyAccount[] = JSON.parse(data);
		let legacy: LegacyData[] = parsed
			.filter((a) => !a.mode)
			.map((a) => {
				return {
					institution: a.comp,
					industry: parseInt(a.type)
				};
			});
		let supers: LegacySuperData[] = parsed
			.filter((a) => a.mode)
			.map((a) => {
				return {
					institution: a.comp,
					industry: parseInt(a.type),
					idendity: a.acc,
					specials: a.spec,
					min: parseInt(a.min),
					max: parseInt(a.max),
					seed: parseInt(a.year) + 2000
				};
			});
		let answer = await ask(
			`Loaded ${legacy.length} legacy-secures and ${supers.length} super-secure accounts. Continue?\n\nThis will try to load all accounts into the database. The added password can be deleted manually.`,
			{
				title: 'My-Passwords: Legacy Data loaded',
				type: 'info'
			}
		);
		if (!answer) return;
		logMsg(
			`Loading legacy data (${supers.length} super-secure and ${legacy.length} legacy-secures) from JSON file...`
		);
		await loadFromJson(legacy, supers).catch(logLoadError('app/list/+page.svelte:load_json'));
		logMsg(`Loaded legacy data`);
	}

	export let data;
</script>

<div class="grid-container">
	<div class="header container">
		<md-icon-button href="/">
			<md-icon>arrow_back</md-icon>
		</md-icon-button>
	</div>

	<div class="list container">
		{#each data.buckets as { id, name }}
			<h3 class="headline">{name}</h3>
			<div class="card-container">
				{#each data.accounts.filter((a) => a.bucket?.name === name) as account}
					<AccountCard {account} />
				{/each}
			</div>
			<div class="actions">
				<md-outlined-button
					on:click={() => triggerDelete(id)}
					on:keypress={() => triggerDelete(id)}
					role="button"
					aria-label="Delete Bucket"
					tabindex="0">Delete Bucket "{name}"</md-outlined-button
				>
			</div>
		{/each}
		<h3 class="headline">Unsorted</h3>
		<div class="card-container">
			{#each data.accounts.filter((a) => !a.bucket) as account}
				<AccountCard {account} />
			{/each}
		</div>
	</div>

	<md-divider style="margin: 32px 0;" />

	<CreateBucket on:created={() => location.reload()} />

	<md-divider style="margin: 32px 0;" />

	<md-outlined-button
		on:click={() => load_json()}
		on:keypress={() => load_json()}
		role="button"
		aria-label="Load Legay data from JSON file"
		tabindex="0">Load Legacy data from JSON file</md-outlined-button
	>
</div>

<style>
	.grid-container {
		display: grid;
		grid-template-columns: 1fr;
		grid-template-rows: auto 1fr;
		height: 100%;
	}
	.card-container {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		grid-gap: 10px;
		grid-auto-flow: row;
		max-width: 100%;
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
	.actions {
		display: flex;
		justify-content: center;
		margin-top: 16px;
		margin-bottom: 100px;
	}
</style>
