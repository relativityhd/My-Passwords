<script lang="ts">
	import { onMount, createEventDispatcher } from 'svelte';
	import '@material/web/button/filled-button';
	import '@material/web/button/text-button';
	import '@material/web/textfield/filled-text-field';
	import '@material/web/select/filled-select';
	import '@material/web/select/select-option';
	import '@material/web/divider/divider';
	import type { MdFilledTextField } from '@material/web/textfield/filled-text-field';
	import type { MdFilledSelect } from '@material/web/select/filled-select';
	import { editSso, type SsoListResult, type Bucket, type SsoOverview } from '$lib/bindings';
	import { goto } from '$app/navigation';

	const dispatch = createEventDispatcher();

	let institution_element: MdFilledTextField;
	let account_element: MdFilledSelect;
	let bucket_element: MdFilledSelect;
	let website_element: MdFilledTextField;
	let recovery_element: MdFilledTextField;

	let isValid = false;

	onMount(() => {
		console.log(account);
		account_element.value = account.ssoaccount_id;
		institution_element.value = account.institution;
		if (account.bucket) {
			bucket_element.value = account.bucket.id;
		} else {
			bucket_element.value = 'unsorted';
		}
		website_element.value = account.website || '';
		recovery_element.value = account.recovery || '';
	});

	function handleInputOnElement(e: MdFilledTextField | MdFilledSelect) {
		async function handleInput() {
			e.reportValidity();
			isValid = institution_element.validity.valid && account_element.validity.valid;
		}
		return handleInput;
	}

	async function handleSubmit() {
		let ssoaccount_id = account_element.value;
		let institution = institution_element.value;
		let bucket: string | null = bucket_element.value;
		if (bucket === 'unsorted') {
			bucket = null;
		}
		let website = website_element.value || null;
		let recovery = recovery_element.value || null;

		let metadata = {
			institution,
			website,
			alias: [],
			recovery
		};

		console.log({ id, metadata, ssoaccount_id, bucket });
		let newacc = await editSso(id, ssoaccount_id, metadata, bucket, null);
		console.log(newacc);
		dispatch('close');
		goto(`/password/sso/${newacc}`);
	}

	export let id: string;
	export let account: SsoOverview;
	export let buckets: Bucket[] = [];
	export let accounts: SsoListResult[] = [];
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
		<md-filled-select
			bind:this={account_element}
			label="Use SSO of"
			disabled={accounts.length === 0}
			on:change={handleInputOnElement(account_element)}
		>
			{#each accounts as { id, institution, bucket }}
				<md-select-option value={id}>
					<div slot="headline">{institution}{bucket ? ' in ' + bucket.name : ''}</div>
				</md-select-option>
			{/each}
		</md-filled-select>
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
		<md-filled-button
			on:click={() => handleSubmit()}
			on:keydown={() => handleSubmit()}
			role="button"
			tabindex="0"
			disabled={!isValid}>Save</md-filled-button
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
