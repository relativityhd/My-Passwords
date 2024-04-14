<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import '@material/web/button/filled-button';
	import '@material/web/button/text-button';
	import '@material/web/textfield/filled-text-field';
	import '@material/web/select/filled-select';
	import '@material/web/select/select-option';
	import type { MdFilledTextField } from '@material/web/textfield/filled-text-field';
	import type { MdFilledSelect } from '@material/web/select/filled-select';
	import { Industry } from '$lib/types';
	// import { liveInput, create } from './bindings';
	import { secureLiveInput, type Bucket, editSecure, type SecureOverview } from '$lib/bindings';
	import { writeText } from '@tauri-apps/api/clipboard';
	import { goto } from '$app/navigation';

	const dispatch = createEventDispatcher();

	let institution_element: MdFilledTextField;
	let account_element: MdFilledTextField;
	let industry_element: MdFilledSelect;
	let bucket_element: MdFilledSelect;
	let website_element: MdFilledTextField;
	let recovery_element: MdFilledTextField;

	onMount(() => {
		console.log(account);
		institution_element.value = account.institution;
		account_element.value = account.identity;
		industry_element.value = account.industry;
		if (account.bucket) {
			bucket_element.value = account.bucket.id;
		}
		website_element.value = account.website || '';
		recovery_element.value = account.recovery || '';
	});

	let password = 'Fill in all fields to see your password';
	let valid = false;
	async function handleInput() {
		let industry = industry_element.value as Industry;
		let institution = institution_element.value;
		let account = account_element.value;
		if (!industry || institution.length === 0 || account.length === 0) {
			password = 'Fill in all fields to see your password';
			valid = false;
			return;
		}
		valid = true;
		password = await secureLiveInput(institution_element.value, account_element.value, industry);
		dispatch('password', password);
	}

	async function handleSubmit(copy: boolean) {
		if (copy) {
			writeText(password);
		}
		let industry = industry_element.value as Industry;
		let institution = institution_element.value;
		let account = account_element.value;
		let bucket = bucket_element.value || null;
		let website = website_element.value || null;
		let recovery = recovery_element.value || null;
		if (!industry || institution.length === 0 || account.length === 0) {
			password = 'Fill in all fields to see your password';
			valid = false;
			return;
		}
		console.log({
			institution,
			account,
			industry,
			bucket,
			website,
			recovery
		});
		let newacc = await editSecure(
			id,
			institution,
			website,
			[],
			account,
			recovery,
			industry,
			bucket,
			null
		);
		console.log(newacc);
		dispatch('close');
		goto(`/password/secure/${newacc}`);
	}

	export let account: SecureOverview;
	export let id: string;
	export let buckets: Bucket[] = [];
</script>

<div>
	<div class="container">
		<md-filled-text-field
			label="Name of institution"
			bind:this={institution_element}
			on:change={handleInput}
			on:input={handleInput}
		/>
		<md-filled-text-field
			label="Your account name"
			bind:this={account_element}
			on:change={handleInput}
			on:input={handleInput}
		/>

		<md-filled-select bind:this={industry_element} on:change={handleInput} label="Industry">
			{#each Object.values(Industry) as industry}
				<md-select-option value={industry}>
					<div slot="headline">{industry}</div>
				</md-select-option>
			{/each}
		</md-filled-select>

		<md-filled-text-field
			label="Institution Website"
			bind:this={website_element}
			on:change={handleInput}
			on:input={handleInput}
		/>
		<md-filled-text-field
			label="Recovery Information"
			bind:this={recovery_element}
			on:change={handleInput}
			on:input={handleInput}
		/>

		<md-filled-select
			bind:this={bucket_element}
			on:change={handleInput}
			label="Bucket"
			disabled={buckets.length === 0}
		>
			<md-select-option value="">
				<div slot="headline">Unsorted</div>
			</md-select-option>
			{#each buckets as { id, name }}
				<md-select-option value={id}>
					<div slot="headline">{name}</div>
				</md-select-option>
			{/each}
		</md-filled-select>
	</div>

	<div class="buttons-row">
		<md-text-button
			on:click={() => dispatch('close')}
			on:keydown={() => dispatch('close')}
			role="button"
			tabindex="0">Close</md-text-button
		>
		<div style="width: 100%;" />
		<md-text-button
			on:click={() => handleSubmit(false)}
			on:keydown={() => handleSubmit(false)}
			role="button"
			tabindex="0"
			disabled={!valid}>Only save</md-text-button
		>
		<md-filled-button
			on:click={() => handleSubmit(true)}
			on:keydown={() => handleSubmit(true)}
			role="button"
			tabindex="0"
			disabled={!valid}>Copy & Save</md-filled-button
		>
	</div>
</div>

<style>
	.container {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		grid-gap: 10px;
		grid-auto-flow: dense;
		margin: 16px 0;
	}

	.buttons-row {
		display: flex;
		justify-content: flex-end;
	}
</style>
