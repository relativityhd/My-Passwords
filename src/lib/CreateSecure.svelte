<script lang="ts">
	import '@material/web/button/filled-button';
	import '@material/web/button/text-button';
	import '@material/web/textfield/filled-text-field';
	import '@material/web/select/filled-select';
	import '@material/web/select/select-option';
	import type { MdFilledTextField } from '@material/web/textfield/filled-text-field';
	import type { MdFilledSelect } from '@material/web/select/filled-select';
	import { Industry } from '$lib/types';
	import { liveInput, create } from './bindings';
	import { writeText } from '@tauri-apps/api/clipboard';
	import { goto } from '$app/navigation';

	let institution_element: MdFilledTextField;
	let account_element: MdFilledTextField;
	let industry_element: MdFilledSelect;
	let bucket_element: MdFilledSelect;
	let website_element: MdFilledTextField;
	let recovery_element: MdFilledTextField;

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
		password = await liveInput(institution_element.value, account_element.value, industry);
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
		}
		let newacc = await create(institution, website, account, recovery, industry, bucket);
		console.log(newacc);
		// goto(`/password/${newacc}`);
	}
	const buckets = ['Bucket 1', 'Bucket 2', 'Bucket 3'];
</script>

<h1>{password}</h1>

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

	<md-filled-text-field label="Institution Website" bind:this={website_element} />
	<md-filled-text-field label="Recovery Information" bind:this={recovery_element} />

	<md-filled-select bind:this={bucket_element} label="Bucket">
		{#each buckets as bucket}
			<md-select-option value={bucket}>
				<div slot="headline">{bucket}</div>
			</md-select-option>
		{/each}
		<!-- TODO: <md-select-option value="new">
			<div slot="headline">New Bucket</div>
		</md-select-option> -->
	</md-filled-select>

	<md-text-button on:click={() => handleSubmit(false)} disabled={!valid}>Only save</md-text-button>
	<md-filled-button on:click={() => handleSubmit(true)} disabled={!valid}
		>Copy & Save</md-filled-button
	>
</div>

<style>
	.container {
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
		grid-auto-flow: row;
		gap: 16px;
	}
</style>
