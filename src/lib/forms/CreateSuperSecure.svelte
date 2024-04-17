<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import '@material/web/button/filled-button';
	import '@material/web/button/text-button';
	import '@material/web/textfield/filled-text-field';
	import '@material/web/select/filled-select';
	import '@material/web/select/select-option';
	import '@material/web/slider/slider';
	import '@material/web/divider/divider';
	import type { MdSlider } from '@material/web/slider/slider';
	import type { MdFilledTextField } from '@material/web/textfield/filled-text-field';
	import type { MdFilledSelect } from '@material/web/select/filled-select';
	import { Industry } from '$lib/types';
	// import { liveInput, create } from './bindings';
	import { supersecureLiveInput, type Bucket, createSupersecure } from '$lib/bindings';
	import { writeText } from '@tauri-apps/api/clipboard';
	import { goto } from '$app/navigation';

	const dispatch = createEventDispatcher();

	let institution_element: MdFilledTextField;
	let account_element: MdFilledTextField;
	let industry_element: MdFilledSelect;
	let specials_element: MdFilledTextField;
	let seed_element: MdFilledTextField;
	let range_element: MdSlider;
	let bucket_element: MdFilledSelect;
	let website_element: MdFilledTextField;
	let recovery_element: MdFilledTextField;

	let isValid = false;

	onMount(() => {
		industry_element.value = Industry.Other;
		bucket_element.value = 'unsorted';
	});

	let password = 'Fill in all fields to see your password';

	function handleInputOnElement(e: MdFilledTextField | MdFilledSelect | null) {
		async function handleInput() {
			if (e) e.reportValidity();

			isValid =
				institution_element.validity.valid &&
				account_element.validity.valid &&
				seed_element.validity.valid;
			if (!isValid) {
				password = 'Fill in all fields to see your password';
				dispatch('password', password);
				return;
			}
			// Values should be okay after this point because of the sanity check above
			let industry = industry_element.value as Industry;
			let institution = institution_element.value;
			let account = account_element.value;
			let specials = specials_element.value;
			let seed = parseInt(seed_element.value);
			let min_length = range_element.valueStart as number;
			let max_length = range_element.valueEnd as number;
			supersecureLiveInput(institution, account, industry, specials, seed, min_length, max_length)
				.then((res) => {
					password = res;
					dispatch('password', password);
				})
				.catch((error) => {
					console.error(error);
					password = error;
					dispatch('password', password);
					isValid = false;
				});
		}
		return handleInput;
	}

	async function handleSubmit(copy: boolean) {
		if (copy) {
			writeText(password);
		}
		let industry = industry_element.value as Industry;
		let institution = institution_element.value;
		let account = account_element.value;
		let bucket: string | null = bucket_element.value;
		if (bucket === 'unsorted') {
			bucket = null;
		}
		let website = website_element.value || null;
		let recovery = recovery_element.value || null;
		let specials = specials_element.value;
		let seed = parseInt(seed_element.value);
		let min_length = range_element.valueStart as number;
		let max_length = range_element.valueEnd as number;

		let metadata = {
			institution,
			website,
			alias: [],
			recovery
		};
		let specifics = {
			identity: account,
			industry,
			specials,
			seed,
			min: min_length,
			max: max_length
		};
		console.log({ metadata, specifics, bucket });
		let newacc = await createSupersecure(metadata, specifics, bucket, null);
		console.log(newacc);
		goto(`/password/supersecure/${newacc}`);
	}
	export let buckets: Bucket[] = [];
</script>

<h3>Mandatory fields</h3>
<div class="container">
	<md-filled-text-field
		label="Name of institution"
		bind:this={institution_element}
		on:change={handleInputOnElement(institution_element)}
		on:input={handleInputOnElement(institution_element)}
		required
	/>
	<md-filled-text-field
		label="Your account name"
		bind:this={account_element}
		on:change={handleInputOnElement(account_element)}
		on:input={handleInputOnElement(account_element)}
		required
	/>

	<md-filled-select
		bind:this={industry_element}
		on:change={handleInputOnElement(industry_element)}
		label="Industry"
	>
		{#each Object.values(Industry) as industry}
			<md-select-option value={industry}>
				<div slot="headline">{industry}</div>
			</md-select-option>
		{/each}
	</md-filled-select>

	<md-filled-text-field
		label="Special Characters"
		bind:this={specials_element}
		value="!#$%&*#-/?@"
		on:change={handleInputOnElement(specials_element)}
		on:input={handleInputOnElement(specials_element)}
	/>
	<md-filled-text-field
		label="Seed"
		bind:this={seed_element}
		value="0"
		min="0"
		max="119"
		type="number"
		required
		on:change={handleInputOnElement(seed_element)}
		on:input={handleInputOnElement(seed_element)}
	/>

	<label for="range">
		Possible Password Length
		<md-slider
			labeled
			id="range"
			bind:this={range_element}
			range
			min="8"
			max="26"
			value-start="10"
			value-end="16"
			on:change={handleInputOnElement(null)}
		/>
	</label>
</div>

<md-divider />

<h3>Metadata</h3>
<div class="container">
	<md-filled-text-field label="Institution Website" bind:this={website_element} />
	<md-filled-text-field label="Recovery Information" bind:this={recovery_element} />
</div>

<md-divider />

<h3>Organisation & 2FA</h3>
<div class="container">
	<md-filled-select bind:this={bucket_element} label="Bucket" disabled={buckets.length === 0}>
		<md-select-option value="unsorted">
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
		on:click={() => handleSubmit(false)}
		on:keydown={() => handleSubmit(false)}
		role="button"
		tabindex="0"
		disabled={!isValid}>Only save</md-text-button
	>
	<md-filled-button
		on:click={() => handleSubmit(true)}
		on:keydown={() => handleSubmit(true)}
		role="button"
		tabindex="0"
		disabled={!isValid}>Copy & Save</md-filled-button
	>
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
