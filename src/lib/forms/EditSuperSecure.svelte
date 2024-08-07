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
	import { Industry, type SerializedError } from '$lib/types';
	import {
		supersecureLiveInput,
		type Bucket,
		editSupersecure,
		type SuperSecureOverview
	} from '$lib/bindings';
	import { writeText } from '@tauri-apps/api/clipboard';
	import { goto } from '$app/navigation';
	import { logLoadError, logMsg } from '$lib/errorutils';

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
		institution_element.value = account.institution;
		account_element.value = account.identity;
		industry_element.value = account.industry;
		specials_element.value = account.specials;
		seed_element.value = account.seed.toString();
		range_element.valueStart = account.min;
		range_element.valueEnd = account.max;

		if (account.bucket) {
			bucket_element.value = account.bucket.id;
		} else {
			bucket_element.value = 'unsorted';
		}
		website_element.value = account.website || '';
		recovery_element.value = account.recovery || '';
	});

	let password = '';

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
			password = await supersecureLiveInput(
				institution,
				account,
				industry,
				specials,
				seed,
				min_length,
				max_length
			).catch((error: SerializedError) => {
				if (error.status !== 400) {
					logLoadError('forms/CreateSuperSecure.svelte:handleInput', {
						institution,
						account,
						industry,
						specials,
						seed,
						min_length,
						max_length
					})(error);
				}
				return "Invalid combination, can't generate password. Please try another set of inputs.";
			});
			dispatch('password', password);
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
		const twofactorid = null;
		logMsg('Editing supersecure account...', { metadata, specifics, bucket, twofactorid });
		let newacc = await editSupersecure(id, metadata, specifics, bucket, twofactorid).catch(
			logLoadError('forms/EditSuperSecure.svelte:handleSubmit', {
				metadata,
				specifics,
				bucket,
				twofactorid
			})
		);
		logMsg('Edited supersecure account with id ' + newacc);
		dispatch('close');
		goto(`/password/supersecure/${newacc}`);
	}

	export let account: SuperSecureOverview;
	export let id: string;
	export let buckets: Bucket[] = [];
</script>

<div class="super">
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
</div>

<style>
	.super {
		max-width: 1000px;
		margin: 0 auto;
	}
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
